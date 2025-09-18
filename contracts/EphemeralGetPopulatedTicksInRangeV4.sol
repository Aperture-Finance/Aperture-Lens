// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {PoolKey} from "@uniswap/v4-core/src/types/PoolKey.sol";

import "./PoolUtilsV4.sol";

/// @notice A lens that fetches chunks of tick data in a range for a Uniswap v4 pool without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (PopulatedTick[]))`
contract EphemeralGetPopulatedTicksInRangeV4 is PoolUtilsV4 {
    using StateLibrary for IPoolManager;

    constructor(address poolManager, PoolKey memory poolKey, int24 tickLower, int24 tickUpper) payable {
        PopulatedTick[] memory populatedTicks = getPopulatedTicksInRange(poolManager, poolKey, tickLower, tickUpper);
        bytes memory returnData = abi.encode(populatedTicks);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @notice Get all the tick data for the populated ticks from tickLower to tickUpper
    /// @param poolManagerAddr The address of the pool manager
    /// @param poolKey The key of the pool to query
    /// @param tickLower The lower tick boundary of the populated ticks to fetch
    /// @param tickUpper The upper tick boundary of the populated ticks to fetch
    /// @return populatedTicks An array of tick data for the given word in the tick bitmap
    function getPopulatedTicksInRange(address poolManagerAddr, PoolKey memory poolKey, int24 tickLower, int24 tickUpper)
        public
        payable
        returns (PopulatedTick[] memory populatedTicks)
    {
        require(tickLower <= tickUpper);
        (int16 wordPosLower, int16 wordPosUpper) = getWordPositions(tickLower, tickUpper, poolKey.tickSpacing);
        unchecked {
            (uint256[] memory tickBitmap, uint256 count) =
                getTickBitmapAndCount(IPoolManager(poolManagerAddr), poolKey, wordPosLower, wordPosUpper);
            // fetch populated tick data
            populatedTicks = new PopulatedTick[](count);
            uint256 idx;
            for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
                idx = populateTicksInWord(
                    poolManagerAddr,
                    poolKey,
                    wordPos,
                    poolKey.tickSpacing,
                    tickBitmap[uint16(wordPos - wordPosLower)],
                    populatedTicks,
                    idx
                );
            }
        }
    }

    /// @notice Get the tick data for all populated ticks in a word of the tick bitmap
    function populateTicksInWord(
        address poolManagerAddr,
        PoolKey memory poolKey,
        int16 wordPos,
        int24 tickSpacing,
        uint256 bitmap,
        PopulatedTick[] memory populatedTicks,
        uint256 idx
    ) internal view returns (uint256) {
        unchecked {
            for (uint256 bitPos; bitPos < 256; ++bitPos) {
                //slither-disable-next-line incorrect-shift
                if (bitmap & (1 << bitPos) != 0) {
                    int24 tick;
                    assembly {
                        tick := mul(tickSpacing, add(shl(8, wordPos), bitPos))
                    }
                    populateTick(poolManagerAddr, poolKey, tick, populatedTicks[idx++]);
                }
            }
            return idx;
        }
    }

    function populateTick(
        address poolManagerAddr,
        PoolKey memory poolKey,
        int24 tick,
        PopulatedTick memory populatedTick
    ) internal view {
        populatedTick.tick = tick;
        (
            populatedTick.liquidityGross,
            populatedTick.liquidityNet,
            populatedTick.feeGrowthOutside0X128,
            populatedTick.feeGrowthOutside1X128
        ) = IPoolManager(poolManagerAddr).getTickInfo(poolKey.toId(), tick);
    }
}
