// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {StateLibrary} from "@uniswap/v4-core/src/libraries/StateLibrary.sol";
import {PoolIdLibrary} from "@uniswap/v4-core/src/types/PoolId.sol";
import {PoolKey} from "@uniswap/v4-core/src/types/PoolKey.sol";
import {IPoolManager} from "@uniswap/v4-core/src/interfaces/IPoolManager.sol";
import {IPositionManager} from "@uniswap/v4-periphery/src/interfaces/IPositionManager.sol";
import {Position} from "@uniswap/v4-core/src/libraries/Position.sol";
import {Currency} from "@uniswap/v4-core/src/types/Currency.sol";
import {FullMath} from "@aperture_finance/uni-v3-lib/src/FullMath.sol";
import {PoolUtilsV4} from "./PoolUtilsV4.sol";
import {PositionFull} from "./PositionUtils.sol";
import {ERC20Callee} from "./libraries/ERC20Caller.sol";
import {PositionInfo, PositionInfoLibrary} from "@uniswap/v4-periphery/src/libraries/PositionInfoLibrary.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

struct Slot0 {
    uint160 sqrtPriceX96;
    int24 tick;
    // For V4-styled exchanges, this is how much fee is paid to the ptotocol.
    uint32 protocolFee;
    // For V4-styled exchanges, this is how much fee is paid to the LP.
    // Only populated for UniV4 and PCS Infinity.
    uint24 lpFee;
}

struct PositionState {
    // token ID of the position
    uint256 tokenId;
    // position's owner
    address owner;
    // nonfungible position manager's position struct with real-time tokensOwed
    PositionFull position;
    // The pool key to look up from v4 pool manager.
    PoolKey poolKey;
    // The pool's fee in hundredths of a bip, i.e. 1e-6
    uint24 poolFee;
    // The pool's tick spacing.
    int24 poolTickSpacing;
    // pool's slot0 struct
    Slot0 slot0;
    // pool's active liquidity
    uint128 activeLiquidity;
    // token0's decimals
    uint8 decimals0;
    // token1's decimals
    uint8 decimals1;
}

/// @title Position utility contract for Uniswap V4
/// @author Aperture Finance
/// @notice Provides utility functions for Uniswap V4 positions using the singleton PoolManager
abstract contract PositionUtilsV4 is PoolUtilsV4 {
    using StateLibrary for IPoolManager;
    using PoolIdLibrary for PoolKey;
    using FullMath for uint128;

    /// @dev Peek a position and calculate the fee growth inside the position
    /// state.position must be populated by `positionInPlace` before calling this function
    /// to further enrich the `PositionState` struct.
    /// @param positionManagerAddr the address of position manager
    /// @param tokenId the ERC721 id of the position
    /// @param state Position state pointer to be updated in place
    function peek(address positionManagerAddr, uint256 tokenId, PositionState memory state) internal view {
        PositionFull memory position = state.position;
        IPositionManager positionManager = IPositionManager(positionManagerAddr);
        IPoolManager poolManager = IPoolManager(positionManager.poolManager());
        (PoolKey memory poolKey,) = positionManager.getPoolAndPositionInfo(tokenId);
        state.tokenId = tokenId;
        state.poolFee = position.feeOrTickSpacing;
        state.poolKey = poolKey;
        state.poolTickSpacing = poolKey.tickSpacing;
        // Pool's liquidity.
        state.activeLiquidity = poolManager.getLiquidity(poolKey.toId());
        // Slot0
        (uint160 sqrtPriceX96, int24 tick, uint24 protocolFee, uint24 lpFee) = poolManager.getSlot0(poolKey.toId());
        state.slot0.sqrtPriceX96 = sqrtPriceX96;
        state.slot0.tick = tick;
        state.slot0.protocolFee = protocolFee;
        state.slot0.lpFee = lpFee;

        (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128) = poolManager.getPositionInfo(
            poolKey.toId(),
            getPositionKeyV4(PositionKeyV4(positionManagerAddr, position.tickLower, position.tickUpper, position.salt))
        );
        if (position.liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
                getFeeGrowthInside(poolManager, poolKey, position.tickLower, position.tickUpper);
            (uint128 tokensOwed0, uint128 tokensOwed1) = calculateFeesGrowth(
                position.liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                feeGrowthInside0LastX128,
                feeGrowthInside1LastX128
            );
            // Direct assignment to overwrite fees
            position.tokensOwed0 = tokensOwed0;
            position.tokensOwed1 = tokensOwed1;
        }
        address address0 = Currency.unwrap(poolKey.currency0);
        address address1 = Currency.unwrap(poolKey.currency1);
        if (address0 == address(0)) {
            state.decimals0 = 18;
        } else {
            state.decimals0 = ERC20Callee.wrap(address0).decimals();
        }

        // Address1 can't be zero.
        state.decimals1 = ERC20Callee.wrap(address1).decimals();
    }

    // Populate basic information: mostly equivalent to V3's npm.positions(tokenId)
    // w/ a few caveats due to V4 architecture.
    /// @param positionManagerAddr Uniswap v4 position manager
    /// @param tokenId ERC 721 token id for the position
    /// @param state The position state
    /// @return exists Whether the position exists
    function positionInPlace(address positionManagerAddr, uint256 tokenId, PositionState memory state)
        internal
        view
        returns (bool exists)
    {
        PositionFull memory position = state.position;
        IPositionManager positionManager = IPositionManager(positionManagerAddr);
        IPoolManager poolManager = IPoolManager(positionManager.poolManager());
        (PoolKey memory poolKey, PositionInfo info) = positionManager.getPoolAndPositionInfo(tokenId);
        // Cast to ERC721 for `ownerOf` call.
        // Note:
        //  There are two owners here:
        //      - The owner of the ERC721 token, which should be the end user.
        //      - The owner of the liquidity position opened through Position Manager
        //          within Pool manager. This owner is the position manager.
        try IERC721(positionManagerAddr).ownerOf(tokenId) returns (address owner) {
            (uint128 liquidity, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128) = poolManager.getPositionInfo(
                poolKey.toId(),
                getPositionKeyV4(
                    PositionKeyV4(positionManagerAddr, info.tickLower(), info.tickUpper(), bytes32(tokenId))
                )
            );

            state.owner = owner;
            position.tickLower = info.tickLower();
            position.tickUpper = info.tickUpper();
            position.liquidity = liquidity;
            position.feeGrowthInside0LastX128 = feeGrowthInside0LastX128;
            position.feeGrowthInside1LastX128 = feeGrowthInside1LastX128;
            position.feeOrTickSpacing = poolKey.fee;
            position.token0 = Currency.unwrap(poolKey.currency0);
            position.token1 = Currency.unwrap(poolKey.currency1);

            // Zero out owed tokens because V4 does not leave positive balance
            // on the book.
            position.tokensOwed0 = 0;
            position.tokensOwed1 = 0;

            // Get operator.
            position.operator = IERC721(positionManagerAddr).getApproved(tokenId);
            position.salt = bytes32(tokenId);
            return true;
        } catch {
            // Position does not exist.
            return false;
        }
    }
}
