// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

// Partial interface for the SlipStream factory. SlipStream factory is named "CLFactory" and "CL" presumably stands for concentrated liquidity.
// https://github.com/velodrome-finance/slipstream/blob/main/contracts/core/interfaces/ICLFactory.sol
interface ISlipStreamCLFactory {
    /// @notice Returns the pool address for a given pair of tokens and a tick spacing, or address 0 if it does not exist
    /// @dev tokenA and tokenB may be passed in either token0/token1 or token1/token0 order
    /// @param tokenA The contract address of either token0 or token1
    /// @param tokenB The contract address of the other token
    /// @param tickSpacing The tick spacing of the pool
    /// @return pool The pool address
    function getPool(address tokenA, address tokenB, int24 tickSpacing) external view returns (address pool);
}
