// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {INonfungiblePositionManager as INPM} from "@aperture_finance/uni-v3-lib/src/interfaces/INonfungiblePositionManager.sol";
import {NPMCaller, PositionFull} from "@aperture_finance/uni-v3-lib/src/NPMCaller.sol";
import {PoolAddress} from "@aperture_finance/uni-v3-lib/src/PoolAddress.sol";
import {IUniswapV3PoolState, V3PoolCallee} from "@aperture_finance/uni-v3-lib/src/PoolCaller.sol";
import {ERC20Callee} from "./libraries/ERC20Caller.sol";
import {PoolUtils} from "./PoolUtils.sol";

struct Slot0 {
    uint160 sqrtPriceX96;
    int24 tick;
    uint16 observationIndex;
    uint16 observationCardinality;
    uint16 observationCardinalityNext;
    uint8 feeProtocol;
    bool unlocked;
}

// The length of the struct is 25 words.
struct PositionState {
    // token ID of the position
    uint256 tokenId;
    // position's owner
    address owner;
    // nonfungible position manager's position struct with real-time tokensOwed
    PositionFull position;
    // pool's slot0 struct
    Slot0 slot0;
    // pool's active liquidity
    uint128 activeLiquidity;
    // token0's decimals
    uint8 decimals0;
    // token1's decimals
    uint8 decimals1;
}

/// @title Position utility contract
/// @author Aperture Finance
/// @notice Base contract for Uniswap v3 that peeks into the current state of position and pool info
abstract contract PositionUtils is PoolUtils {
    /// @dev Peek a position and calculate the fee growth inside the position
    /// state.position must be populated before calling this function
    /// @param npm Nonfungible position manager
    /// @param tokenId Token ID of the position
    /// @param state Position state pointer to be updated in place
    function peek(INPM npm, uint256 tokenId, PositionState memory state) internal view {
        state.tokenId = tokenId;
        PositionFull memory position = state.position;
        V3PoolCallee pool = V3PoolCallee.wrap(
            PoolAddress.computeAddressSorted(NPMCaller.factory(npm), position.token0, position.token1, position.fee)
        );
        state.activeLiquidity = pool.liquidity();
        slot0InPlace(pool, state.slot0);
        if (position.liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) = getFeeGrowthInside(
                pool,
                position.tickLower,
                position.tickUpper,
                state.slot0.tick
            );
            (uint128 fees0, uint128 fees1) = calculateFeesGrowth(
                position.liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                position.feeGrowthInside0LastX128,
                position.feeGrowthInside1LastX128
            );
            position.tokensOwed0 += fees0;
            position.tokensOwed1 += fees1;
        }
        state.decimals0 = ERC20Callee.wrap(position.token0).decimals();
        state.decimals1 = ERC20Callee.wrap(position.token1).decimals();
    }

    /// @dev Equivalent to `INonfungiblePositionManager.positions(tokenId)`
    /// @param npm Uniswap v3 Nonfungible Position Manager
    /// @param tokenId The ID of the token that represents the position
    /// @param pos The position pointer to be updated in place
    function positionInPlace(INPM npm, uint256 tokenId, PositionFull memory pos) internal view returns (bool exists) {
        bytes4 selector = INPM.positions.selector;
        assembly ("memory-safe") {
            // Write the abi-encoded calldata into memory.
            mstore(0, selector)
            mstore(4, tokenId)
            // We use 36 because of the length of our calldata.
            // We copy up to 384 bytes of return data at pos's pointer.
            exists := staticcall(gas(), npm, 0, 0x24, pos, 0x180)
        }
    }

    /// @dev Equivalent to `IUniswapV3Pool.slot0`
    /// @param pool Uniswap v3 pool
    /// @param s Slot0 pointer to be updated in place
    function slot0InPlace(V3PoolCallee pool, Slot0 memory s) internal view {
        bytes4 selector = IUniswapV3PoolState.slot0.selector;
        assembly ("memory-safe") {
            // Write the function selector into memory.
            mstore(0, selector)
            // We use 4 because of the length of our calldata.
            // We copy up to 224 bytes of return data after fmp.
            if iszero(staticcall(gas(), pool, 0, 4, s, 0xe0)) {
                revert(0, 0)
            }
        }
    }
}
