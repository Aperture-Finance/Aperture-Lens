// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {LiquidityAmounts} from "@aperture_finance/uni-v3-lib/src/LiquidityAmounts.sol";
import {PoolCaller, V3PoolCallee} from "@aperture_finance/uni-v3-lib/src/PoolCaller.sol";
import {TickMath} from "@aperture_finance/uni-v3-lib/src/TickMath.sol";
import {PoolUtils} from "./PoolUtils.sol";

/// @title Position lens contract
/// @author Aperture Finance
/// @notice Provides functions for fetching fee amounts and total amounts of a position
contract PositionLens is PoolUtils {
    using TickMath for int24;

    /// @notice Returns the fees owed to a position
    /// @param pool Uniswap v3 pool
    /// @param owner The address of the position owner
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @return tokensOwed0 The amount of token0 owed to the position
    /// @return tokensOwed1 The amount of token1 owed to the position
    function getFeesOwed(
        V3PoolCallee pool,
        address owner,
        int24 tickLower,
        int24 tickUpper
    ) public view returns (uint128 tokensOwed0, uint128 tokensOwed1) {
        bytes32 key = getPositionKey(PositionKey(owner, tickLower, tickUpper));
        PoolCaller.PositionInfo memory info = pool.positions(key);
        (, int24 tickCurrent) = pool.sqrtPriceX96AndTick();

        if (info.liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) = getFeeGrowthInside(
                pool,
                tickLower,
                tickUpper,
                tickCurrent
            );
            (tokensOwed0, tokensOwed1) = calculateFeesGrowth(
                info.liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                info.feeGrowthInside0LastX128,
                info.feeGrowthInside1LastX128
            );
        }
    }

    /// @notice Returns the total amounts of token0 and token1 held in a position including fees
    /// @param pool Uniswap v3 pool
    /// @param owner The address of the position owner
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @return amount0 The total amount of token0 held in the position
    /// @return amount1 The total amount of token1 held in the position
    function getTotalAmounts(
        V3PoolCallee pool,
        address owner,
        int24 tickLower,
        int24 tickUpper
    ) external view returns (uint256 amount0, uint256 amount1) {
        bytes32 key = getPositionKey(PositionKey(owner, tickLower, tickUpper));
        PoolCaller.PositionInfo memory info = pool.positions(key);
        (uint160 sqrtPriceX96, ) = pool.sqrtPriceX96AndTick();

        if (info.liquidity != 0) {
            (amount0, amount1) = LiquidityAmounts.getAmountsForLiquidity(
                sqrtPriceX96,
                tickLower.getSqrtRatioAtTick(),
                tickUpper.getSqrtRatioAtTick(),
                info.liquidity
            );
            (uint256 fees0, uint256 fees1) = getFeesOwed(pool, owner, tickLower, tickUpper);
            amount0 += fees0;
            amount1 += fees1;
        }
    }
}
