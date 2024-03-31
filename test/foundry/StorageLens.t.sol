// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@pancakeswap/v3-core/contracts/interfaces/IPancakeV3Factory.sol";
import "contracts/EphemeralStorageLens.sol";
import "./Base.t.sol";

contract StorageLensTest is BaseTest {
    function setUp() public virtual override {
        super.setUp();
        vm.etch(pool, vm.getDeployedCode("EphemeralStorageLens.sol"));
    }

    function test_extsload() public {
        uint256 length = 10;
        bytes32[] memory slots = new bytes32[](length);
        for (uint256 i; i < length; ++i) {
            slots[i] = bytes32(i);
        }
        bytes32[] memory values = EphemeralStorageLens(pool).extsload(slots);
        for (uint256 i; i < length; ++i) {
            assertEq(values[i], vm.load(pool, slots[i]));
        }
    }

    /// forge-config: default.fuzz.runs = 16
    /// forge-config: ci.fuzz.runs = 16
    function testFuzz_extsload(bytes32 slot) public {
        bytes32[] memory slots = new bytes32[](1);
        slots[0] = slot;
        bytes32[] memory values = EphemeralStorageLens(pool).extsload(slots);
        assertEq(values[0], vm.load(pool, slot));
    }
}

contract PCSV3StorageLensTest is StorageLensTest {
    function setUp() public override {
        dex = DEX.PancakeSwapV3;
        super.setUp();
    }
}
