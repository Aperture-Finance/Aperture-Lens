// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {FullMath} from "@aperture_finance/uni-v3-lib/src/FullMath.sol";

import {CLPosition} from "infinity-core/src/pool-cl/libraries/CLPosition.sol";
import {Tick} from "infinity-core/src/pool-cl/libraries/Tick.sol";
import {TickBitmap} from "infinity-core/src/pool-cl/libraries/TickBitmap.sol";
import {PoolKey} from "infinity-core/src/types/PoolKey.sol";
import {PoolIdLibrary} from "infinity-core/src/types/PoolId.sol";
import {ICLPoolManager} from "infinity-core/src/pool-cl/interfaces/ICLPoolManager.sol";

import {LibBit} from "solady/src/utils/LibBit.sol";

/// @title Pool utility contract for PancakeSwap Inifnity
/// @author Aperture Finance
/// @notice Provides utility functions for PCS Inifinity pools
abstract contract PoolUtilsPCSV4 {
    using PoolIdLibrary for PoolKey;
    using FullMath for uint128;

    uint256 internal constant Q128 = 1 << 128;

    /// @notice Position key for V4 (includes salt)
    struct PositionKeyV4 {
        address owner;
        int24 tickLower;
        int24 tickUpper;
        bytes32 salt;
    }

    /// @notice Populated tick data for infinity
    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }

    /// @notice Returns the position key hash for V4 positions (includes salt)
    /// @param key The position key containing owner, tick range, and salt
    /// @return positionKey The hashed position key
    function getPositionKeyV4(PositionKeyV4 memory key) internal pure returns (bytes32 positionKey) {
        return CLPosition.calculatePositionKey(key.owner, key.tickLower, key.tickUpper, key.salt);
    }

    /// @notice Returns the tick bitmap keys of the given tick range
    function getWordPositions(int24 tickLower, int24 tickUpper, int24 tickSpacing)
        internal
        pure
        returns (int16 wordPosLower, int16 wordPosUpper)
    {
        int24 compressed = TickBitmap.compress(tickLower, tickSpacing);
        wordPosLower = int16(compressed >> 8);
        compressed = TickBitmap.compress(tickUpper, tickSpacing);
        wordPosUpper = int16(compressed >> 8);
    }

    /// @notice Retrieves real time fee growth data from ticks.
    /// @param pool the PCS Infinity pool manager address
    /// @param poolKey the PCS Infinity pool key
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @return feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
    /// @return feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
    function getFeeGrowthInside(ICLPoolManager pool, PoolKey memory poolKey, int24 tickLower, int24 tickUpper)
        internal
        view
        returns (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128)
    {
        // Get current tick.
        (, int24 currentTick,,) = pool.getSlot0(poolKey.toId());

        (uint256 feeGrowthGlobal0X128, uint256 feeGrowthGlobal1X128) = pool.getFeeGrowthGlobals(poolKey.toId());

        // Compute fee growth outside for tick lower and upper.
        Tick.Info memory tickLowerInfo = pool.getPoolTickInfo(poolKey.toId(), tickLower);
        Tick.Info memory tickUpperInfo = pool.getPoolTickInfo(poolKey.toId(), tickUpper);

        uint256 tickLowerFeeGrowthOutside0X128 = tickLowerInfo.feeGrowthOutside0X128;
        uint256 tickLowerFeeGrowthOutside1X128 = tickLowerInfo.feeGrowthOutside1X128;
        uint256 tickUpperFeeGrowthOutside0X128 = tickUpperInfo.feeGrowthOutside0X128;
        uint256 tickUpperFeeGrowthOutside1X128 = tickUpperInfo.feeGrowthOutside1X128;

        // Compute fee growth inside.
        if (currentTick < tickLower) {
            feeGrowthInside0X128 = tickLowerFeeGrowthOutside0X128 - tickUpperFeeGrowthOutside0X128;
            feeGrowthInside1X128 = tickLowerFeeGrowthOutside1X128 - tickUpperFeeGrowthOutside1X128;
        } else if (currentTick >= tickUpper) {
            feeGrowthInside0X128 = tickUpperFeeGrowthOutside0X128 - tickLowerFeeGrowthOutside0X128;
            feeGrowthInside1X128 = tickUpperFeeGrowthOutside1X128 - tickLowerFeeGrowthOutside1X128;
        } else {
            feeGrowthInside0X128 =
                feeGrowthGlobal0X128 - tickLowerFeeGrowthOutside0X128 - tickUpperFeeGrowthOutside0X128;
            feeGrowthInside1X128 =
                feeGrowthGlobal1X128 - tickLowerFeeGrowthOutside1X128 - tickUpperFeeGrowthOutside1X128;
        }
    }

    /// @notice Calculate fees owed to a position (same logic as V3)
    /// @param liquidity Position liquidity
    /// @param feeGrowthInside0X128 Current fee growth inside range for token0
    /// @param feeGrowthInside1X128 Current fee growth inside range for token1
    /// @param feeGrowthInside0LastX128 Last recorded fee growth for token0
    /// @param feeGrowthInside1LastX128 Last recorded fee growth for token1
    /// @return tokensOwed0 Fees owed in token0
    /// @return tokensOwed1 Fees owed in token1
    function calculateFeesGrowth(
        uint128 liquidity,
        uint256 feeGrowthInside0X128,
        uint256 feeGrowthInside1X128,
        uint256 feeGrowthInside0LastX128,
        uint256 feeGrowthInside1LastX128
    ) internal pure returns (uint128 tokensOwed0, uint128 tokensOwed1) {
        unchecked {
            tokensOwed0 = uint128(liquidity.mulDiv(feeGrowthInside0X128 - feeGrowthInside0LastX128, Q128));
            tokensOwed1 = uint128(liquidity.mulDiv(feeGrowthInside1X128 - feeGrowthInside1LastX128, Q128));
        }
    }

    /// @notice Cache the bitmap and calculate the number of populated ticks
    /// @param pool Uniswap v4 pool manager
    /// @param poolKey The key that uniquely identifies the pool within the manager
    /// @param wordPosLower The lower word position of the tick bitmap
    /// @param wordPosUpper The upper word position of the tick bitmap
    /// @return tickBitmap The tick bitmap
    /// @return count The number of populated ticks
    function getTickBitmapAndCount(ICLPoolManager pool, PoolKey memory poolKey, int16 wordPosLower, int16 wordPosUpper)
        internal
        view
        returns (uint256[] memory tickBitmap, uint256 count)
    {
        tickBitmap = new uint256[](uint16(wordPosUpper - wordPosLower + 1));
        for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
            uint256 bitmap = pool.getPoolBitmapInfo(poolKey.toId(), wordPos);
            tickBitmap[uint16(wordPos - wordPosLower)] = bitmap;
            count += LibBit.popCount(bitmap);
        }
    }
}
