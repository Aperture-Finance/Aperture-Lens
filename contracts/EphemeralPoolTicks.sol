// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PoolUtils.sol";

/// @notice A lens that fetches raw storage slots of the `ticks` mapping in a range for a Uniswap v3 pool without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (Slot[]))`
contract EphemeralPoolTicks is PoolUtils {
    constructor(V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable {
        Slot[] memory slots = getPopulatedTicksInRange(pool, tickLower, tickUpper);
        bytes memory returnData = abi.encode(slots);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    function getTicksSlot() internal pure virtual returns (uint256) {
        // Storage slot of the `ticks` mapping in UniswapV3Pool.
        return 5;
    }

    /// @notice Get all the tick data for the populated ticks from tickLower to tickUpper
    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param pool The address of the pool for which to fetch populated tick data
    /// @param tickLower The lower tick boundary of the populated ticks to fetch
    /// @param tickUpper The upper tick boundary of the populated ticks to fetch
    /// @return slots An array of storage slots and their raw data
    function getPopulatedTicksInRange(
        V3PoolCallee pool,
        int24 tickLower,
        int24 tickUpper
    ) public payable returns (Slot[] memory slots) {
        require(tickLower <= tickUpper);
        // checks that the pool exists
        int24 tickSpacing = IUniswapV3Pool(V3PoolCallee.unwrap(pool)).tickSpacing();
        (int16 wordPosLower, int16 wordPosUpper) = getWordPositions(tickLower, tickUpper, tickSpacing);
        unchecked {
            (uint256[] memory tickBitmap, uint256 count) = getTickBitmapAndCount(pool, wordPosLower, wordPosUpper);
            // each tick occupies 4 storage slots
            slots = new Slot[](count << 2);
            // fetch populated tick data
            uint256 idx;
            for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
                idx = populateTicksInWord(
                    pool,
                    wordPos,
                    tickSpacing,
                    tickBitmap[uint16(wordPos - wordPosLower)],
                    slots,
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
        Slot[] memory slots,
        uint256 idx
    ) internal view returns (uint256) {
        unchecked {
            uint256 TICKS_SLOT = getTicksSlot();
            for (uint256 bitPos; bitPos < 256; ++bitPos) {
                //slither-disable-next-line incorrect-shift
                if (bitmap & (1 << bitPos) != 0) {
                    int24 tick;
                    assembly {
                        tick := mul(tickSpacing, add(shl(8, wordPos), bitPos))
                    }
                    PoolCaller.TickInfo memory info = pool.ticks(tick);
                    // calculate the storage slot corresponding to the tick
                    // the slot of ticks[tick] is keccak256(abi.encode(tick, ticks.slot))
                    uint256 slot;
                    assembly ("memory-safe") {
                        mstore(0, tick)
                        mstore(0x20, TICKS_SLOT)
                        slot := keccak256(0, 0x40)
                    }
                    {
                        uint256 data;
                        assembly ("memory-safe") {
                            let liquidityGross := mload(info)
                            let liquidityNet := mload(add(info, 0x20))
                            data := or(shl(128, liquidityNet), liquidityGross)
                        }
                        slots[idx++] = Slot(slot++, data);
                    }
                    slots[idx++] = Slot(slot++, info.feeGrowthOutside0X128);
                    slots[idx++] = Slot(slot++, info.feeGrowthOutside1X128);
                    {
                        uint256 data;
                        assembly ("memory-safe") {
                            let initialized := mload(add(info, 0xe0))
                            data := shl(248, initialized)
                            let secondsOutside := mload(add(info, 0xc0))
                            data := or(shl(216, secondsOutside), data)
                            let secondsPerLiquidityOutsideX128 := mload(add(info, 0xa0))
                            data := or(shl(56, secondsPerLiquidityOutsideX128), data)
                            let tickCumulativeOutside := and(0xffffffffffffff, mload(add(info, 0x80)))
                            data := or(tickCumulativeOutside, data)
                        }
                        slots[idx++] = Slot(slot, data);
                    }
                }
            }
            return idx;
        }
    }
}

contract EphemeralPCSV3PoolTicks is EphemeralPoolTicks {
    constructor(
        V3PoolCallee pool,
        int24 tickLower,
        int24 tickUpper
    ) payable EphemeralPoolTicks(pool, tickLower, tickUpper) {}

    function getTicksSlot() internal pure override returns (uint256) {
        // Storage slot of the `ticks` mapping in PancakeSwapV3Pool.
        return 6;
    }
}
