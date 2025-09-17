// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {StateLibrary} from "@uniswap/v4-core/src/libraries/StateLibrary.sol";
import {PoolId} from "@uniswap/v4-core/src/types/PoolId.sol";
import {PoolKey} from "@uniswap/v4-core/src/types/PoolKey.sol";
import {IPoolManager} from "@uniswap/v4-core/src/interfaces/IPoolManager.sol";
import {LiquidityAmounts} from "@aperture_finance/uni-v3-lib/src/LiquidityAmounts.sol";
import {TickMath} from "@aperture_finance/uni-v3-lib/src/TickMath.sol";
import {PoolUtilsV4} from "./PoolUtilsV4.sol";

/// @title Position lens contract for Uniswap V4
/// @author Aperture Finance
/// @notice Provides functions for fetching fee amounts and total amounts of V4 positions
contract PositionLensV4 is PoolUtilsV4 {
    using TickMath for int24;
    using StateLibrary for IPoolManager;

    /// @notice Returns the fees owed to a V4 position
    /// @param poolManager The PoolManager contract
    /// @param poolKey The pool key identifying the pool
    /// @param owner The address of the position owner
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @param salt The salt used for position identification
    /// @return tokensOwed0 The amount of token0 owed to the position
    /// @return tokensOwed1 The amount of token1 owed to the position
    function getFeesOwed(
        IPoolManager poolManager,
        PoolKey memory poolKey,
        address owner,
        int24 tickLower,
        int24 tickUpper,
        bytes32 salt
    ) public view returns (uint128 tokensOwed0, uint128 tokensOwed1) {
        (uint128 liquidity, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128) = poolManager
            .getPositionInfo(poolKey.toId(), getPositionKeyV4(PositionKeyV4(owner, tickLower, tickUpper, salt)));
        if (liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
                getFeeGrowthInside(poolManager, poolKey, tickLower, tickUpper);

            (tokensOwed0, tokensOwed1) = calculateFeesGrowth(
                liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                feeGrowthInside0LastX128,
                feeGrowthInside1LastX128
            );
        }
    }

    /// @notice Returns the total amounts of token0 and token1 held in a V4 position including fees
    /// @param pm The PoolManager contract
    /// @param poolKey The pool key identifying the pool
    /// @param owner The address of the position owner
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @param salt The salt used for position identification
    /// @return amount0 The total amount of token0 held in the position
    /// @return amount1 The total amount of token1 held in the position
    function getTotalAmounts(
        IPoolManager pm,
        PoolKey memory poolKey,
        address owner,
        int24 tickLower,
        int24 tickUpper,
        bytes32 salt
    ) public view returns (uint256 amount0, uint256 amount1) {
        (uint128 liquidity,,) = pm.getPositionInfo(poolKey.toId(), owner, tickLower, tickUpper, salt);
        (uint160 sqrtPriceX96,,,) = pm.getSlot0(poolKey.toId());

        if (liquidity != 0) {
            (amount0, amount1) = LiquidityAmounts.getAmountsForLiquidity(
                sqrtPriceX96, tickLower.getSqrtRatioAtTick(), tickUpper.getSqrtRatioAtTick(), liquidity
            );

            (uint256 fees0, uint256 fees1) = getFeesOwed(pm, poolKey, owner, tickLower, tickUpper, salt);
            amount0 += fees0;
            amount1 += fees1;
        }
    }
}
