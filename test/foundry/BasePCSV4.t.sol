// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {Currency} from "infinity-core/src/types/Currency.sol";
import {ICLPoolManager} from "infinity-core/src/pool-cl/interfaces/ICLPoolManager.sol";
import {ICLHooks} from "infinity-core/src/pool-cl/interfaces/ICLHooks.sol";
import {PoolKey} from "infinity-core/src/types/PoolKey.sol";
import {CLPoolParametersHelper} from "infinity-core/src/pool-cl/libraries/CLPoolParametersHelper.sol";

// Base V4 related tests on Ethereum mainnet.
abstract contract BasePCSV4Test is Test {
    // Ethereum mainnet pool manager address.
    address internal constant poolManagerAddr = 0xa0FfB9c1CE1Fe56963B0321B32E7A0302114058b;
    address internal constant posManagerAddr = 0x55f4c8abA71A1e923edC303eb4fEfF14608cC226;
    address internal constant USDC = 0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d;

    // V4 uses zero address and it will always be the token0.
    address token0 = address(0);
    address token1 = USDC;
    PoolKey internal poolKey = PoolKey(
        Currency.wrap(token0),
        Currency.wrap(token1),
        ICLHooks(0x32C59D556B16DB81DFc32525eFb3CB257f7e493d),
        ICLPoolManager(poolManagerAddr),
        8388608, // Fee tier
        0x00000000000000000000000000000000000000000000000000000000000a00c2 // params
    );

    function setUp() public virtual {
        vm.createSelectFork("bnb", 70172173);
    }

    function currentTick() public view returns (int24) {
        (, int24 tick,,) = ICLPoolManager(poolManagerAddr).getSlot0(poolKey.toId());
        return tick;
    }
}
