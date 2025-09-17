// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "contracts/EphemeralGetPopulatedTicksInRangeV4.sol";
import "./BaseV4.t.sol";

contract TickLensV4Test is BaseV4Test, PoolUtilsV4 {
    using StateLibrary for IPoolManager;

    function verifyTicks(PopulatedTick[] memory populatedTicks) internal view {
        for (uint256 i; i < populatedTicks.length; ++i) {
            PopulatedTick memory populatedTick = populatedTicks[i];
            (uint128 liquidityGross, int128 liquidityNet, uint256 feeGrowthOutside0X128, uint256 feeGrowthOutside1X128)
            = IPoolManager(poolManagerAddr).getTickInfo(poolKey.toId(), populatedTick.tick);
            assertEq(liquidityGross, populatedTick.liquidityGross, "liquidityGross");
            assertEq(liquidityNet, populatedTick.liquidityNet, "liquidityNet");
            assertEq(feeGrowthOutside0X128, populatedTick.feeGrowthOutside0X128, "feeGrowthOutside0X128");
            assertEq(feeGrowthOutside1X128, populatedTick.feeGrowthOutside1X128, "feeGrowthOutside1X128");
        }
    }

    function test_GetPopulatedTicksInRangeV4() public {
        int24 tick = currentTick();
        int24 width = 10;
        try new EphemeralGetPopulatedTicksInRangeV4(
            poolManagerAddr, poolKey, tick - width * poolKey.tickSpacing, tick + width * poolKey.tickSpacing
        ) {} catch (bytes memory returnData) {
            PopulatedTick[] memory populatedTicks = abi.decode(returnData, (PopulatedTick[]));
            console2.log("length", populatedTicks.length);
            verifyTicks(populatedTicks);
        }
    }
}
