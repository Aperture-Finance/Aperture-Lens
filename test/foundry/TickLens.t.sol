// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@pancakeswap/v3-core/contracts/interfaces/IPancakeV3Factory.sol";
import "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import "contracts/EphemeralGetPopulatedTicksInRange.sol";
import "./Base.t.sol";

contract TickLensTest is BaseTest, PoolUtils {
    function verifyTicks(PopulatedTick[] memory populatedTicks) internal view {
        for (uint256 i; i < populatedTicks.length; ++i) {
            PopulatedTick memory populatedTick = populatedTicks[i];
            (
                uint128 liquidityGross,
                int128 liquidityNet,
                uint256 feeGrowthOutside0X128,
                uint256 feeGrowthOutside1X128,
                ,
                ,
                ,

            ) = IUniswapV3Pool(pool).ticks(populatedTick.tick);
            assertEq(liquidityGross, populatedTick.liquidityGross, "liquidityGross");
            assertEq(liquidityNet, populatedTick.liquidityNet, "liquidityNet");
            assertEq(feeGrowthOutside0X128, populatedTick.feeGrowthOutside0X128, "feeGrowthOutside0X128");
            assertEq(feeGrowthOutside1X128, populatedTick.feeGrowthOutside1X128, "feeGrowthOutside1X128");
        }
    }

    function test_GetPopulatedTicksInRange() public {
        int24 tick = currentTick();
        try
            new EphemeralGetPopulatedTicksInRange(
                V3PoolCallee.wrap(pool),
                tick - 128 * tickSpacing,
                tick + 128 * tickSpacing
            )
        {} catch (bytes memory returnData) {
            PopulatedTick[] memory populatedTicks = abi.decode(returnData, (PopulatedTick[]));
            console2.log("length", populatedTicks.length);
            verifyTicks(populatedTicks);
        }
    }
}

contract PCSV3TickLensTest is TickLensTest {
    function setUp() public override {
        dex = DEX.PancakeSwapV3;
        super.setUp();
    }
}
