// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@pancakeswap/v3-core/contracts/interfaces/IPancakeV3Factory.sol";
import "contracts/EphemeralPoolSlots.sol";
import "contracts/EphemeralPoolTicks.sol";
import "contracts/EphemeralPoolTickBitmap.sol";
import "contracts/EphemeralPoolPositions.sol";
import "./Base.t.sol";

contract PoolLensTest is BaseTest, PoolUtils {
    function test_GetSlots() public {
        try new EphemeralPoolSlots(V3PoolCallee.wrap(pool)) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }

    function test_GetPopulatedTicksInRange() public {
        int24 tick = currentTick();
        try new EphemeralPoolTicks(V3PoolCallee.wrap(pool), tick, tick) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }

    function test_GetTickBitmap() public {
        try new EphemeralPoolTickBitmap(V3PoolCallee.wrap(pool)) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }

    function test_GetPositions() public {
        int24 tick = currentTick();
        testFuzz_GetPositions(tick - tickSpacing, tick + tickSpacing);
    }

    /// forge-config: default.fuzz.runs = 16
    /// forge-config: ci.fuzz.runs = 16
    function testFuzz_GetPositions(int24 tickLower, int24 tickUpper) public {
        (tickLower, tickUpper) = prepTicks(tickLower, tickUpper);
        uint256 amount0Desired = token0Unit;
        uint256 amount1Desired = token1Unit;
        deal(token0, address(this), type(uint256).max);
        deal(token1, address(this), type(uint256).max);
        PositionKey[] memory keys = new PositionKey[](3);
        for (uint256 i; i < 3; ++i) {
            mint(address(this), amount0Desired, amount1Desired, tickLower, tickUpper);
            keys[i] = PositionKey(address(this), tickLower, tickUpper);
            tickLower -= tickSpacing;
            tickUpper += tickSpacing;
        }
        try new EphemeralPoolPositions(V3PoolCallee.wrap(pool), keys) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }
}

contract PCSV3PoolLensTest is BaseTest, PoolUtils {
    function setUp() public override {
        dex = DEX.PancakeSwapV3;
        super.setUp();
    }

    function test_GetSlots() public {
        try new EphemeralPCSV3PoolSlots(V3PoolCallee.wrap(pool)) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }

    function test_GetPopulatedTicksInRange() public {
        int24 tick = currentTick();
        try new EphemeralPCSV3PoolTicks(V3PoolCallee.wrap(pool), tick, tick) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }

    function test_GetTickBitmap() public {
        try new EphemeralPCSV3PoolTickBitmap(V3PoolCallee.wrap(pool)) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }

    function test_GetPositions() public {
        int24 tick = currentTick();
        testFuzz_GetPositions(tick - tickSpacing, tick + tickSpacing);
    }

    /// forge-config: default.fuzz.runs = 16
    /// forge-config: ci.fuzz.runs = 16
    function testFuzz_GetPositions(int24 tickLower, int24 tickUpper) public {
        (tickLower, tickUpper) = prepTicks(tickLower, tickUpper);
        uint256 amount0Desired = token0Unit;
        uint256 amount1Desired = token1Unit;
        deal(token0, address(this), type(uint256).max);
        deal(token1, address(this), type(uint256).max);
        PositionKey[] memory keys = new PositionKey[](3);
        for (uint256 i; i < 3; ++i) {
            mint(address(this), amount0Desired, amount1Desired, tickLower, tickUpper);
            keys[i] = PositionKey(address(this), tickLower, tickUpper);
            tickLower -= tickSpacing;
            tickUpper += tickSpacing;
        }
        try new EphemeralPCSV3PoolPositions(V3PoolCallee.wrap(pool), keys) {} catch (bytes memory returnData) {
            Slot[] memory slots = abi.decode(returnData, (Slot[]));
            uint256 length = slots.length;
            for (uint256 i; i < length; ++i) {
                assertEq(bytes32(slots[i].data), vm.load(pool, bytes32(slots[i].slot)));
            }
        }
    }
}
