// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {Currency} from "@uniswap/v4-core/src/types/Currency.sol";
import {IPoolManager} from "@uniswap/v4-core/src/interfaces/IPoolManager.sol";
import {IHooks} from "@uniswap/v4-core/src/interfaces/IHooks.sol";
import {PoolKey} from "@uniswap/v4-core/src/types/PoolKey.sol";
import {StateLibrary} from "@uniswap/v4-core/src/libraries/StateLibrary.sol";

// Base V4 related tests on Ethereum mainnet.
abstract contract BaseV4Test is Test {
    using StateLibrary for IPoolManager;

    // Ethereum mainnet pool manager address.
    address internal constant poolManagerAddr = 0x000000000004444c5dc75cB358380D2e3dE08A90;
    address internal constant posManagerAddr = 0xbD216513d74C8cf14cf4747E6AaA6420FF64ee9e;
    address internal constant USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;

    // V4 uses zero address and it will always be the token0.
    address token0 = address(0);
    address token1 = USDC;
    PoolKey internal poolKey = PoolKey(Currency.wrap(token0), Currency.wrap(token1), 500, 10, IHooks(address(0)));

    function setUp() public virtual {
        vm.createSelectFork("ethereum", 23371710);
    }

    function currentTick() public view returns (int24) {
        (, int24 tick,,) = IPoolManager(poolManagerAddr).getSlot0(poolKey.toId());
        return tick;
    }
}
