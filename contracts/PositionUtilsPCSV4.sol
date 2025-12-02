// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;
import {FullMath} from "@aperture_finance/uni-v3-lib/src/FullMath.sol";

import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

import {PoolKey as V4PoolKey} from "@uniswap/v4-core/src/types/PoolKey.sol";
import {Currency as V4Currency} from "@uniswap/v4-core/src/types/Currency.sol";
import {IHooks as V4IHooks} from "@uniswap/v4-core/src/interfaces/IHooks.sol";

import {PositionState} from "./PositionUtilsV4.sol";

import {CLPosition} from "infinity-core/src/pool-cl/libraries/CLPosition.sol";
import {Tick} from "infinity-core/src/pool-cl/libraries/Tick.sol";
import {TickBitmap} from "infinity-core/src/pool-cl/libraries/TickBitmap.sol";
import {PoolKey} from "infinity-core/src/types/PoolKey.sol";
import {PoolIdLibrary} from "infinity-core/src/types/PoolId.sol";
import {ICLPoolManager} from "infinity-core/src/pool-cl/interfaces/ICLPoolManager.sol";
import {ICLPositionManager} from "infinity-periphery/src/pool-cl/interfaces/ICLPositionManager.sol";
import {
    CLPositionInfo,
    CLPositionInfoLibrary
} from "infinity-periphery/src/pool-cl/libraries/CLPositionInfoLibrary.sol";
import {Currency} from "infinity-core/src/types/Currency.sol";
import {CLPoolParametersHelper} from "infinity-core/src/pool-cl/libraries/CLPoolParametersHelper.sol";
import {IHooks} from "infinity-core/src/interfaces/IHooks.sol";

import {PoolUtilsPCSV4} from "./PoolUtilsPCSV4.sol";
import {PositionFull} from "./PositionUtils.sol";
import {ERC20Callee} from "./libraries/ERC20Caller.sol";
import {Slot0} from "./PositionUtils.sol";

/// @title Position utility contract for PCS Infinity
/// @author Aperture Finance
/// @notice Provides utility functions for PCS Infinity positions using the singleton PoolManager
abstract contract PositionUtilsPCSV4 is PoolUtilsPCSV4 {
    using PoolIdLibrary for PoolKey;
    using CLPositionInfoLibrary for CLPositionInfo;
    using FullMath for uint128;

    /// @dev Peek a position and calculate the fee growth inside the position
    /// state.position must be populated by `positionInPlace` before calling this function
    /// to further enrich the `PositionState` struct.
    /// @param positionManagerAddr the address of position manager
    /// @param tokenId the ERC721 id of the position
    /// @param state Position state pointer to be updated in place
    function peek(address positionManagerAddr, uint256 tokenId, PositionState memory state) internal view {
        PositionFull memory position = state.position;
        ICLPositionManager positionManager = ICLPositionManager(positionManagerAddr);
        ICLPoolManager poolManager = ICLPoolManager(positionManager.clPoolManager());
        (PoolKey memory poolKey,) = positionManager.getPoolAndPositionInfo(tokenId);
        state.tokenId = tokenId;
        state.poolFee = position.feeOrTickSpacing;
        state.poolTickSpacing = CLPoolParametersHelper.getTickSpacing(poolKey.parameters);
        // Fit PCS V4 poolKey into V4 PoolKey.
        state.poolKey = V4PoolKey(
            V4Currency.wrap(Currency.unwrap(poolKey.currency0)),
            V4Currency.wrap(Currency.unwrap(poolKey.currency1)),
            poolKey.fee,
            state.poolTickSpacing,
            V4IHooks(address(poolKey.hooks))
        );
        // Pool's liquidity.
        state.activeLiquidity = poolManager.getLiquidity(poolKey.toId());
        // Slot0
        (uint160 sqrtPriceX96, int24 tick, uint24 protocolFee,) = poolManager.getSlot0(poolKey.toId());
        state.slot0.sqrtPriceX96 = sqrtPriceX96;
        state.slot0.tick = tick;
        state.slot0.feeProtocol = protocolFee;

        CLPosition.Info memory positionInfo = poolManager.getPosition(
            poolKey.toId(), positionManagerAddr, position.tickLower, position.tickUpper, position.salt
        );
        if (position.liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
                getFeeGrowthInside(poolManager, poolKey, position.tickLower, position.tickUpper);
            (uint128 tokensOwed0, uint128 tokensOwed1) = calculateFeesGrowth(
                position.liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                positionInfo.feeGrowthInside0LastX128,
                positionInfo.feeGrowthInside1LastX128
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
    // w/ a few caveats due to V4/Infinity architecture.
    /// @param positionManagerAddr PCS Infinity position manager
    /// @param tokenId ERC 721 token id for the position
    /// @param state The position state
    /// @return exists Whether the position exists
    function positionInPlace(address positionManagerAddr, uint256 tokenId, PositionState memory state)
        internal
        view
        returns (bool exists)
    {
        PositionFull memory position = state.position;
        ICLPositionManager positionManager = ICLPositionManager(positionManagerAddr);
        ICLPoolManager poolManager = ICLPoolManager(positionManager.clPoolManager());
        (PoolKey memory poolKey, CLPositionInfo info) = positionManager.getPoolAndPositionInfo(tokenId);
        // Cast to ERC721 for `ownerOf` call.
        // Note:
        //  There are two owners here:
        //      - The owner of the ERC721 token, which should be the end user.
        //      - The owner of the liquidity position opened through Position Manager
        //          within Pool manager. This owner is the position manager.
        try IERC721(positionManagerAddr).ownerOf(tokenId) returns (address owner) {
            CLPosition.Info memory positionInfo = poolManager.getPosition(
                poolKey.toId(), positionManagerAddr, info.tickLower(), info.tickUpper(), bytes32(tokenId)
            );
            state.owner = owner;
            position.tickLower = info.tickLower();
            position.tickUpper = info.tickUpper();
            position.liquidity = positionInfo.liquidity;
            position.feeGrowthInside0LastX128 = positionInfo.feeGrowthInside0LastX128;
            position.feeGrowthInside1LastX128 = positionInfo.feeGrowthInside1LastX128;
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
