// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {
    ICommonNonfungiblePositionManager as INPM,
    IUniswapV3NonfungiblePositionManager as IUniV3NPM
} from "@aperture_finance/uni-v3-lib/src/interfaces/IUniswapV3NonfungiblePositionManager.sol";
import {
    IPCSV3NonfungiblePositionManager as IPCSV3NPM
} from "@aperture_finance/uni-v3-lib/src/interfaces/IPCSV3NonfungiblePositionManager.sol";
import {
    ISlipStreamNonfungiblePositionManager as ISlipStreamNPM
} from "@aperture_finance/uni-v3-lib/src/interfaces/ISlipStreamNonfungiblePositionManager.sol";
import {PoolAddress} from "@aperture_finance/uni-v3-lib/src/PoolAddress.sol";
import {PoolAddressPancakeSwapV3} from "@aperture_finance/uni-v3-lib/src/PoolAddressPancakeSwapV3.sol";
import {IUniswapV3PoolState, V3PoolCallee} from "@aperture_finance/uni-v3-lib/src/PoolCaller.sol";
import {IUniswapV3Factory} from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol";
import {IPancakeV3Factory} from "@pancakeswap/v3-core/contracts/interfaces/IPancakeV3Factory.sol";
import {ERC20Callee} from "./libraries/ERC20Caller.sol";
import {PoolUtils} from "./PoolUtils.sol";
import {DEX} from "./Dex.sol";
import "./interfaces/ISlipStreamCLFactory.sol";

struct Slot0 {
    uint160 sqrtPriceX96;
    int24 tick;
    uint16 observationIndex;
    uint16 observationCardinality;
    uint16 observationCardinalityNext;
    // `feeProtocol` is of type uint8 in Uniswap V3, and uint32 in PancakeSwap V3. We use uint32 here as this can hold both uint8 and uint32.
    // `feeProtocol` doesn't exist on SlipStream so this is manually set to 0.
    uint32 feeProtocol;
    // This should always be true because it is only temporarily set to false as re-entrancy guard during a transaction.
    bool unlocked;
}

struct PositionFull {
    // the nonce for permits
    // for v4, this field will be missing due to its unordered nonce design.
    uint96 nonce;
    // the address that is approved for spending this token
    address operator;
    address token0;
    address token1;
    // The pool's fee or tickSpacing. This depends on the DEX's NPM.positions() implementation. This is `fee` for Uniswap V3 and PancakeSwap V3, and `tickSpacing` for SlipStream.
    uint24 feeOrTickSpacing;
    // the tick range of the position
    int24 tickLower;
    int24 tickUpper;
    // the liquidity of the position
    uint128 liquidity;
    // the fee growth of the aggregate position as of the last action on the individual position
    uint256 feeGrowthInside0LastX128;
    uint256 feeGrowthInside1LastX128;
    // how many uncollected tokens are owed to the position, as of the last computation
    uint128 tokensOwed0;
    uint128 tokensOwed1;

    // Only populated for UniV4.
    bytes32 salt;
}

struct PositionState {
    // token ID of the position
    uint256 tokenId;
    // position's owner
    address owner;
    // nonfungible position manager's position struct with real-time tokensOwed
    PositionFull position;
    // The pool address.
    address pool;
    // The pool's fee in hundredths of a bip, i.e. 1e-6
    uint24 poolFee;
    // The pool's tick spacing.
    int24 poolTickSpacing;
    // pool's slot0 struct
    Slot0 slot0;
    // pool's active liquidity
    uint128 activeLiquidity;
    // token0's decimals
    uint8 decimals0;
    // token1's decimals
    uint8 decimals1;
}

/// @title Position utility contract
/// @author Aperture Finance
/// @notice Base contract for Uniswap v3 that peeks into the current state of position and pool info
abstract contract PositionUtils is PoolUtils {
    /// @dev Peek a position and calculate the fee growth inside the position
    /// state.position must be populated before calling this function
    /// @param dex DEX type
    /// @param npm Nonfungible position manager
    /// @param tokenId Token ID of the position
    /// @param state Position state pointer to be updated in place
    function peek(DEX dex, address npm, uint256 tokenId, PositionState memory state) internal view {
        state.tokenId = tokenId;
        PositionFull memory position = state.position;
        if (dex == DEX.UniswapV3) {
            state.poolFee = position.feeOrTickSpacing;
            state.pool =
                IUniswapV3Factory(IUniV3NPM(npm).factory()).getPool(position.token0, position.token1, state.poolFee);
        } else if (dex == DEX.PancakeSwapV3) {
            state.poolFee = position.feeOrTickSpacing;
            state.pool =
                IPancakeV3Factory(IUniV3NPM(npm).factory()).getPool(position.token0, position.token1, state.poolFee);
        } else if (dex == DEX.SlipStream) {
            state.poolTickSpacing = int24(position.feeOrTickSpacing);
            state.pool = ISlipStreamCLFactory(ISlipStreamNPM(npm).factory())
                .getPool(position.token0, position.token1, state.poolTickSpacing);
        }
        V3PoolCallee pool = V3PoolCallee.wrap(state.pool);
        state.poolFee = pool.fee();
        state.poolTickSpacing = pool.tickSpacing();
        state.activeLiquidity = pool.liquidity();
        slot0InPlace(pool, state.slot0);
        // Manually adjust `feeProtocol` and `unlocked` fields for SlipStream as `feeProtocol` doesn't exist on SlipStream.
        if (dex == DEX.SlipStream) {
            state.slot0.feeProtocol = 0;
            state.slot0.unlocked = true;
        }
        if (position.liquidity != 0) {
            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
                getFeeGrowthInside(dex, state.pool, position.tickLower, position.tickUpper, state.slot0.tick);
            (uint128 fees0, uint128 fees1) = calculateFeesGrowth(
                position.liquidity,
                feeGrowthInside0X128,
                feeGrowthInside1X128,
                position.feeGrowthInside0LastX128,
                position.feeGrowthInside1LastX128
            );
            position.tokensOwed0 += fees0;
            position.tokensOwed1 += fees1;
        }
        state.decimals0 = ERC20Callee.wrap(position.token0).decimals();
        state.decimals1 = ERC20Callee.wrap(position.token1).decimals();
    }

    /// @dev Equivalent to `INonfungiblePositionManager.positions(tokenId)`
    /// @param npm Uniswap v3 Nonfungible Position Manager
    /// @param tokenId The ID of the token that represents the position
    /// @param pos The position pointer to be updated in place
    /// @return exists Whether the position exists
    function positionInPlace(address npm, uint256 tokenId, PositionFull memory pos)
        internal
        view
        returns (bool exists)
    {
        bytes4 selector = IUniV3NPM(npm).positions.selector;
        assembly ("memory-safe") {
            // Write the abi-encoded calldata into memory.
            mstore(0, selector)
            mstore(4, tokenId)
            // We use 36 because of the length of our calldata.
            // We copy up to 384 bytes of return data at pos's pointer.
            exists := staticcall(gas(), npm, 0, 0x24, pos, 0x180)
        }
    }

    /// @dev Equivalent to `IUniswapV3Pool.slot0`
    /// @param pool Uniswap v3 pool
    /// @param slot0 Slot0 pointer to be updated in place
    function slot0InPlace(V3PoolCallee pool, Slot0 memory slot0) internal view {
        bytes4 selector = IUniswapV3PoolState.slot0.selector;
        assembly ("memory-safe") {
            // Write the function selector into memory.
            mstore(0, selector)
            // We use 4 because of the length of our calldata.
            // We copy up to 224 bytes of return data at slot0's pointer.
            if iszero(staticcall(gas(), pool, 0, 4, slot0, 0xe0)) {
                revert(0, 0)
            }
        }
    }
}
