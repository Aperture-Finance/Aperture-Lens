// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "contracts/PositionUtils.sol";
import "@aperture_finance/uni-v3-lib/src/LiquidityAmounts.sol";
import "@aperture_finance/uni-v3-lib/src/PoolCaller.sol";
import "@aperture_finance/uni-v3-lib/src/TernaryLib.sol";
import "@aperture_finance/uni-v3-lib/src/TickBitmap.sol";
import "@aperture_finance/uni-v3-lib/src/TickMath.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@pancakeswap/v3-core/contracts/interfaces/callback/IPancakeV3MintCallback.sol";
import "@pancakeswap/v3-core/contracts/interfaces/callback/IPancakeV3SwapCallback.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import "@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3MintCallback.sol";
import "@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol";
import "forge-std/Test.sol";
import "solady/src/utils/SafeTransferLib.sol";

abstract contract BaseTest is
    Test,
    IPancakeV3MintCallback,
    IPancakeV3SwapCallback,
    IUniswapV3MintCallback,
    IUniswapV3SwapCallback
{
    using SafeTransferLib for address;
    using TernaryLib for bool;
    using TickMath for int24;

    /// @dev Literal numbers used in sqrtPriceLimitX96 = zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1
    /// = (MAX_SQRT_RATIO - 1) ^ ((MIN_SQRT_RATIO + 1 ^ MAX_SQRT_RATIO - 1) * zeroForOne)
    uint160 internal constant MIN_SQRT_RATIO_PLUS_ONE = TickMath.MIN_SQRT_RATIO + 1;
    uint160 internal constant MAX_SQRT_RATIO_MINUS_ONE = 1461446703485210103287273052203988822378723970342 - 1;

    DEX internal dex;
    address internal npm;

    address internal WETH;
    address internal constant USDC = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;
    address internal token0;
    address internal token1;
    uint24 internal constant fee = 500;

    address internal factory;
    address internal pool;
    uint256 internal token0Unit;
    uint256 internal token1Unit;
    int24 internal tickSpacing;

    // Configure state variables for each chain after creating a fork
    function initAfterFork() internal {
        factory = INPM(npm).factory();
        WETH = INPM(npm).WETH9();
        (token0, token1) = (WETH < USDC).switchIf(USDC, WETH);
        pool = IUniswapV3Factory(factory).getPool(token0, token1, fee);
        tickSpacing = V3PoolCallee.wrap(pool).tickSpacing();
        token0Unit = 10 ** IERC20Metadata(token0).decimals();
        token1Unit = 10 ** IERC20Metadata(token1).decimals();
    }

    function setUp() public virtual {
        vm.createSelectFork("base", 17577000);
        if (dex == DEX.UniswapV3) {
            npm = 0x03a520b32C04BF3bEEf7BEb72E919cf822Ed34f1;
        } else if (dex == DEX.PancakeSwapV3) {
            npm = 0x46A15B0b27311cedF172AB29E4f4766fbE7F4364;
        } else {
            npm = 0x827922686190790b37229fd06084350E74485b72;
        }
        initAfterFork();
        vm.label(WETH, "WETH");
        vm.label(USDC, "USDC");
        vm.label(address(npm), "NPM");
        vm.label(pool, "Pool");
        vm.label(address(this), "Test");
    }

    /************************************************
     *  HELPERS
     ***********************************************/

    function sqrtPriceX96() internal view returns (uint160 sqrtRatioX96) {
        (sqrtRatioX96, ) = V3PoolCallee.wrap(pool).sqrtPriceX96AndTick();
    }

    function currentTick() internal view returns (int24 tick) {
        (, tick) = V3PoolCallee.wrap(pool).sqrtPriceX96AndTick();
    }

    /// @dev Normalize tick to align with tick spacing
    function matchSpacing(int24 tick) internal view returns (int24) {
        int24 _tickSpacing = tickSpacing;
        return TickBitmap.compress(tick, _tickSpacing) * _tickSpacing;
    }

    /// @dev Normalize ticks to be valid
    function prepTicks(int24 tickLower, int24 tickUpper) internal view returns (int24, int24) {
        int24 MIN_TICK = matchSpacing(TickMath.MIN_TICK) + tickSpacing;
        int24 MAX_TICK = matchSpacing(TickMath.MAX_TICK);
        tickLower = matchSpacing(int24(bound(tickLower, MIN_TICK, MAX_TICK)));
        tickUpper = matchSpacing(int24(bound(tickUpper, MIN_TICK, MAX_TICK)));
        if (tickLower > tickUpper) (tickLower, tickUpper) = (tickUpper, tickLower);
        else if (tickLower == tickUpper) tickUpper += tickSpacing;
        return (tickLower, tickUpper);
    }

    /************************************************
     *  ACTIONS
     ***********************************************/

    /// @dev Pay pool to finish swap
    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata) external {
        if (amount0Delta > 0) token0.safeTransfer(pool, uint256(amount0Delta));
        if (amount1Delta > 0) token1.safeTransfer(pool, uint256(amount1Delta));
    }

    /// @dev Pay pool to finish minting
    function uniswapV3MintCallback(uint256 amount0Owed, uint256 amount1Owed, bytes calldata) external {
        if (amount0Owed > 0) token0.safeTransfer(pool, amount0Owed);
        if (amount1Owed > 0) token1.safeTransfer(pool, amount1Owed);
    }

    /// @dev Pay pool to finish swap
    function pancakeV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata) external {
        if (amount0Delta > 0) token0.safeTransfer(pool, uint256(amount0Delta));
        if (amount1Delta > 0) token1.safeTransfer(pool, uint256(amount1Delta));
    }

    /// @dev Pay pool to finish minting
    function pancakeV3MintCallback(uint256 amount0Owed, uint256 amount1Owed, bytes calldata) external {
        if (amount0Owed > 0) token0.safeTransfer(pool, amount0Owed);
        if (amount1Owed > 0) token1.safeTransfer(pool, amount1Owed);
    }

    /// @dev Make a direct pool mint
    function mint(
        address recipient,
        uint256 amount0Desired,
        uint256 amount1Desired,
        int24 tickLower,
        int24 tickUpper
    ) internal returns (bool success) {
        uint128 liquidity = LiquidityAmounts.getLiquidityForAmounts(
            sqrtPriceX96(),
            tickLower.getSqrtRatioAtTick(),
            tickUpper.getSqrtRatioAtTick(),
            amount0Desired,
            amount1Desired
        );
        try IUniswapV3Pool(pool).mint(recipient, tickLower, tickUpper, liquidity, new bytes(0)) returns (
            uint256,
            uint256
        ) {
            success = true;
        } catch Error(string memory reason) {
            // `mint` may fail if liquidity is too high or tick range is too narrow
            assertEq(reason, "LO", "only catch liquidity overflow");
        }
    }

    /// @dev Swap twice and return to the initial price to generate some fees
    function swapBackAndForth(uint256 amountIn, bool zeroForOne, uint160 initialPrice) internal {
        if (zeroForOne) {
            (, int256 amount1) = V3PoolCallee.wrap(pool).swap(
                address(this),
                true,
                int256(amountIn),
                MIN_SQRT_RATIO_PLUS_ONE,
                new bytes(0)
            );
            assembly {
                amountIn := mul(2, sub(0, amount1))
            }
            // Swap back to the initial price
            V3PoolCallee.wrap(pool).swap(address(this), false, int256(amountIn), initialPrice, new bytes(0));
        } else {
            (int256 amount0, ) = V3PoolCallee.wrap(pool).swap(
                address(this),
                false,
                int256(amountIn),
                MAX_SQRT_RATIO_MINUS_ONE,
                new bytes(0)
            );
            assembly {
                amountIn := mul(2, sub(0, amount0))
            }
            // Swap back to the initial price
            V3PoolCallee.wrap(pool).swap(address(this), true, int256(amountIn), initialPrice, new bytes(0));
        }
    }
}
