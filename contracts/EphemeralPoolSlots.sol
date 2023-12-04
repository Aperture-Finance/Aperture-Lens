// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PoolUtils.sol";

/// @notice A lens for fetching static state variables in a Uniswap v3 pool without deployment
/// @author Aperture Finance
/// @dev The return data can be accessed externally by `eth_call` without a `to` address or internally by catching the
/// revert data, and decoded by `abi.decode(data, (Slot[]))`
contract EphemeralPoolSlots is PoolUtils {
    constructor(V3PoolCallee pool) payable {
        Slot[] memory slots = getSlots(pool);
        bytes memory returnData = abi.encode(slots);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @notice Get the static storage slots of a pool
    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param pool The Uniswap v3 pool
    /// @return slots An array of storage slots and their raw data
    function getSlots(V3PoolCallee pool) public payable returns (Slot[] memory slots) {
        unchecked {
            uint256 length;
            {
                (
                    uint160 sqrtPriceX96,
                    int24 tick,
                    uint16 observationIndex,
                    uint16 observationCardinality,
                    uint16 observationCardinalityNext,
                    uint8 feeProtocol,
                    bool unlocked
                ) = pool.slot0();
                uint256 slot0;
                assembly {
                    slot0 := shl(240, unlocked)
                    slot0 := or(shl(232, feeProtocol), slot0)
                    slot0 := or(shl(216, observationCardinalityNext), slot0)
                    slot0 := or(shl(200, observationCardinality), slot0)
                    slot0 := or(shl(184, observationIndex), slot0)
                    slot0 := or(shl(160, and(0xffffff, tick)), slot0)
                    slot0 := or(sqrtPriceX96, slot0)
                }
                length = observationCardinality;
                slots = new Slot[](length + 5);
                slots[0] = Slot(0, slot0);
            }
            slots[1] = Slot(1, pool.feeGrowthGlobal0X128());
            slots[2] = Slot(2, pool.feeGrowthGlobal1X128());
            {
                (uint128 token0, uint128 token1) = pool.protocolFees();
                uint256 slot3;
                assembly {
                    slot3 := or(shl(128, token1), token0)
                }
                slots[3] = Slot(3, slot3);
            }
            slots[4] = Slot(4, pool.liquidity());
            for (uint256 i; i < length; ++i) {
                (
                    uint32 blockTimestamp,
                    int56 tickCumulative,
                    uint160 secondsPerLiquidityCumulativeX128,
                    bool initialized
                ) = pool.observations(i);
                uint256 observation;
                assembly {
                    observation := shl(248, initialized)
                    observation := or(shl(88, secondsPerLiquidityCumulativeX128), observation)
                    observation := or(shl(32, and(0xffffffffffffff, tickCumulative)), observation)
                    observation := or(blockTimestamp, observation)
                }
                slots[i + 5] = Slot(i + OBSERVATIONS_SLOT, observation);
            }
        }
    }
}
