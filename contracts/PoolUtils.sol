// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {FullMath} from "@aperture_finance/uni-v3-lib/src/FullMath.sol";
import {IUniswapV3Pool, PoolCaller, V3PoolCallee} from "@aperture_finance/uni-v3-lib/src/PoolCaller.sol";
import {TickBitmap} from "@aperture_finance/uni-v3-lib/src/TickBitmap.sol";
import {LibBit} from "solady/src/utils/LibBit.sol";

/// @title Pool utility contract
/// @author Aperture Finance
/// @notice Provides utility functions for Uniswap v3 pools
abstract contract PoolUtils {
    using FullMath for uint128;

    uint256 internal constant Q128 = 1 << 128;

    /// @notice Struct for a storage slot and its raw data
    struct Slot {
        uint256 slot;
        uint256 data;
    }

    /// @notice Position key
    struct PositionKey {
        address owner;
        int24 tickLower;
        int24 tickUpper;
    }

    /// @notice Populated tick data
    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }

    /// @notice Returns the position key of the given owner and tick range
    /// @param key owner The address of the position owner
    /// tickLower The lower tick boundary of the position
    /// tickUpper The upper tick boundary of the position
    /// @return positionKey The position key of the given owner and tick range
    function getPositionKey(PositionKey memory key) internal pure returns (bytes32 positionKey) {
        // positionKey = keccak256(abi.encodePacked(owner, tickLower, tickUpper))
        assembly ("memory-safe") {
            mstore(0x06, mload(add(key, 0x40))) // [0x23, 0x26)
            mstore(0x03, mload(add(key, 0x20))) // [0x20, 0x23)
            mstore(0, mload(key)) // [0x0c, 0x20)
            positionKey := keccak256(0x0c, 0x1a)
        }
    }

    /// @notice Returns the tick bitmap keys of the given tick range
    function getWordPositions(
        int24 tickLower,
        int24 tickUpper,
        int24 tickSpacing
    ) internal pure returns (int16 wordPosLower, int16 wordPosUpper) {
        int24 compressed = TickBitmap.compress(tickLower, tickSpacing);
        wordPosLower = int16(compressed >> 8);
        compressed = TickBitmap.compress(tickUpper, tickSpacing);
        wordPosUpper = int16(compressed >> 8);
    }

    /// @notice Retrieves fee growth data
    /// @param pool Uniswap v3 pool
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @param tickCurrent The current tick
    /// @return feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
    /// @return feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
    function getFeeGrowthInside(
        V3PoolCallee pool,
        int24 tickLower,
        int24 tickUpper,
        int24 tickCurrent
    ) internal view returns (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) {
        PoolCaller.TickInfo memory lower = pool.ticks(tickLower);
        PoolCaller.TickInfo memory upper = pool.ticks(tickUpper);

        unchecked {
            if (tickCurrent < tickLower) {
                feeGrowthInside0X128 = lower.feeGrowthOutside0X128 - upper.feeGrowthOutside0X128;
                feeGrowthInside1X128 = lower.feeGrowthOutside1X128 - upper.feeGrowthOutside1X128;
            } else if (tickCurrent >= tickUpper) {
                feeGrowthInside0X128 = upper.feeGrowthOutside0X128 - lower.feeGrowthOutside0X128;
                feeGrowthInside1X128 = upper.feeGrowthOutside1X128 - lower.feeGrowthOutside1X128;
            } else {
                feeGrowthInside0X128 =
                    pool.feeGrowthGlobal0X128() -
                    lower.feeGrowthOutside0X128 -
                    upper.feeGrowthOutside0X128;
                feeGrowthInside1X128 =
                    pool.feeGrowthGlobal1X128() -
                    lower.feeGrowthOutside1X128 -
                    upper.feeGrowthOutside1X128;
            }
        }
    }

    /// @notice Calculates the fees growth
    /// @param liquidity The liquidity of the position
    /// @param feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
    /// @param feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
    /// @param feeGrowthInside0LastX128 The all-time fee growth in token0, per unit of liquidity, as of the last update to liquidity or fees owed
    /// @param feeGrowthInside1LastX128 The all-time fee growth in token1, per unit of liquidity, as of the last update to liquidity or fees owed
    /// @return tokensOwed0 The amount of token0 owed to the position as fees
    /// @return tokensOwed1 The amount of token1 owed to the position as fees
    function calculateFeesGrowth(
        uint128 liquidity,
        uint256 feeGrowthInside0X128,
        uint256 feeGrowthInside1X128,
        uint256 feeGrowthInside0LastX128,
        uint256 feeGrowthInside1LastX128
    ) internal pure returns (uint128 tokensOwed0, uint128 tokensOwed1) {
        unchecked {
            // calculate accumulated fees
            tokensOwed0 = uint128(liquidity.mulDiv(feeGrowthInside0X128 - feeGrowthInside0LastX128, Q128));
            tokensOwed1 = uint128(liquidity.mulDiv(feeGrowthInside1X128 - feeGrowthInside1LastX128, Q128));
        }
    }

    /// @notice Cache the bitmap and calculate the number of populated ticks
    /// @param pool Uniswap v3 pool
    /// @param wordPosLower The lower word position of the tick bitmap
    /// @param wordPosUpper The upper word position of the tick bitmap
    /// @return tickBitmap The tick bitmap
    /// @return count The number of populated ticks
    function getTickBitmapAndCount(
        V3PoolCallee pool,
        int16 wordPosLower,
        int16 wordPosUpper
    ) internal view returns (uint256[] memory tickBitmap, uint256 count) {
        tickBitmap = new uint256[](uint16(wordPosUpper - wordPosLower + 1));
        for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
            uint256 bitmap = pool.tickBitmap(wordPos);
            tickBitmap[uint16(wordPos - wordPosLower)] = bitmap;
            count += LibBit.popCount(bitmap);
        }
    }
}
