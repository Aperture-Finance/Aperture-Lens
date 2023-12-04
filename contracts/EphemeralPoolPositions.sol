// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PoolUtils.sol";

/// @notice A lens that batches fetching of the `positions` mapping for a Uniswap v3 pool without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (Slot[]))`
contract EphemeralPoolPositions is PoolUtils {
    constructor(V3PoolCallee pool, PositionKey[] memory keys) payable {
        Slot[] memory slots = getPositions(pool, keys);
        bytes memory returnData = abi.encode(slots);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @notice Get liquidity positions in a pool
    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param pool The address of the pool for which to fetch the tick bitmap
    /// @param keys The position keys to fetch
    /// @return slots An array of storage slots and their raw data
    function getPositions(V3PoolCallee pool, PositionKey[] memory keys) public payable returns (Slot[] memory slots) {
        unchecked {
            uint256 length = keys.length;
            // each position occupies 4 storage slots
            slots = new Slot[](length << 2);
            uint256 j;
            for (uint256 i; i < length; ++i) {
                // calculate the storage slot corresponding to the position key
                // the slot of positions[key] is keccak256(abi.encode(key, positions.slot))
                bytes32 key = getPositionKey(keys[i]);
                uint256 slot;
                assembly ("memory-safe") {
                    mstore(0, key)
                    mstore(0x20, POSITIONS_SLOT)
                    slot := keccak256(0, 0x40)
                }
                PoolCaller.PositionInfo memory info = pool.positions(key);
                slots[j++] = Slot(slot++, info.liquidity);
                slots[j++] = Slot(slot++, info.feeGrowthInside0LastX128);
                slots[j++] = Slot(slot++, info.feeGrowthInside1LastX128);
                uint256 data;
                assembly {
                    data := or(shl(128, mload(add(info, 0x80))), mload(add(info, 0x60)))
                }
                slots[j++] = Slot(slot, data);
            }
        }
    }
}
