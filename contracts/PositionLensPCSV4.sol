// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {LiquidityAmounts} from "@aperture_finance/uni-v3-lib/src/LiquidityAmounts.sol";
import {TickMath} from "@aperture_finance/uni-v3-lib/src/TickMath.sol";

import {CLPosition} from "infinity-core/src/pool-cl/libraries/CLPosition.sol";
import {Tick} from "infinity-core/src/pool-cl/libraries/Tick.sol";
import {TickBitmap} from "infinity-core/src/pool-cl/libraries/TickBitmap.sol";
import {PoolKey} from "infinity-core/src/types/PoolKey.sol";
import {PoolIdLibrary} from "infinity-core/src/types/PoolId.sol";
import {ICLPoolManager} from "infinity-core/src/pool-cl/interfaces/ICLPoolManager.sol";

import {PoolUtilsPCSV4} from "./PoolUtilsPCSV4.sol";

/// @title Position lens contract for PCS Infinity
/// @author Aperture Finance
/// @notice Provides functions for fetching fee amounts and total amounts of PCS Infinity positions
contract PositionLensPCSV4 is PoolUtilsPCSV4 {
    using TickMath for int24;

    /// @notice Returns the fees owed to a PCS Infinity position
    /// @param poolManager The PoolManager contract
    /// @param poolKey The pool key identifying the pool
    /// @param owner The address of the position owner
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @param salt The salt used for position identification
    /// @return tokensOwed0 The amount of token0 owed to the position
    /// @return tokensOwed1 The amount of token1 owed to the position
    function getFeesOwed(
        ICLPoolManager poolManager,
        PoolKey memory poolKey,
        address owner,
        int24 tickLower,
        int24 tickUpper,
        bytes32 salt
    ) public view returns (uint128 tokensOwed0, uint128 tokensOwed1) {
        CLPosition.Info memory positionInfo = poolManager.getPosition(poolKey.toId(), owner, tickLower, tickUpper, salt);
        if (positionInfo.liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
                getFeeGrowthInside(poolManager, poolKey, tickLower, tickUpper);

            (tokensOwed0, tokensOwed1) = calculateFeesGrowth(
                positionInfo.liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                positionInfo.feeGrowthInside0LastX128,
                positionInfo.feeGrowthInside1LastX128
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
        ICLPoolManager pm,
        PoolKey memory poolKey,
        address owner,
        int24 tickLower,
        int24 tickUpper,
        bytes32 salt
    ) public view returns (uint256 amount0, uint256 amount1) {
        uint128 liquidity = pm.getPosition(poolKey.toId(), owner, tickLower, tickUpper, salt).liquidity;
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
