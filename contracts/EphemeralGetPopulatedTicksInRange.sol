// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./PoolUtils.sol";

/// @notice A lens that fetches chunks of tick data in a range for a Uniswap v3 pool without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (PopulatedTick[]))`
contract EphemeralGetPopulatedTicksInRange is PoolUtils {
    constructor(V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable {
        PopulatedTick[] memory populatedTicks = getPopulatedTicksInRange(pool, tickLower, tickUpper);
        bytes memory returnData = abi.encode(populatedTicks);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @notice Get all the tick data for the populated ticks from tickLower to tickUpper
    /// @param pool The address of the pool for which to fetch populated tick data
    /// @param tickLower The lower tick boundary of the populated ticks to fetch
    /// @param tickUpper The upper tick boundary of the populated ticks to fetch
    /// @return populatedTicks An array of tick data for the given word in the tick bitmap
    function getPopulatedTicksInRange(
        V3PoolCallee pool,
        int24 tickLower,
        int24 tickUpper
    ) public payable returns (PopulatedTick[] memory populatedTicks) {
        require(tickLower <= tickUpper);
        // checks that the pool exists
        int24 tickSpacing = IUniswapV3Pool(V3PoolCallee.unwrap(pool)).tickSpacing();
        (int16 wordPosLower, int16 wordPosUpper) = getWordPositions(tickLower, tickUpper, tickSpacing);
        unchecked {
            (uint256[] memory tickBitmap, uint256 count) = getTickBitmapAndCount(pool, wordPosLower, wordPosUpper);
            // fetch populated tick data
            populatedTicks = new PopulatedTick[](count);
            uint256 idx;
            for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
                idx = populateTicksInWord(
                    pool,
                    wordPos,
                    tickSpacing,
                    tickBitmap[uint16(wordPos - wordPosLower)],
                    populatedTicks,
                    idx
                );
            }
        }
    }

    /// @notice Get the tick data for all populated ticks in a word of the tick bitmap
    function populateTicksInWord(
        V3PoolCallee pool,
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
                    populateTick(pool, tick, populatedTicks[idx++]);
                }
            }
            return idx;
        }
    }

    function populateTick(V3PoolCallee pool, int24 tick, PopulatedTick memory populatedTick) internal view {
        PoolCaller.TickInfo memory info = pool.ticks(tick);
        populatedTick.tick = tick;
        populatedTick.liquidityNet = info.liquidityNet;
        populatedTick.liquidityGross = info.liquidityGross;
        populatedTick.feeGrowthOutside0X128 = info.feeGrowthOutside0X128;
        populatedTick.feeGrowthOutside1X128 = info.feeGrowthOutside1X128;
    }
}
