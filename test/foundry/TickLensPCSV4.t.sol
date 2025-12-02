// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Tick} from "infinity-core/src/pool-cl/libraries/Tick.sol";
import {CLPoolParametersHelper} from "infinity-core/src/pool-cl/libraries/CLPoolParametersHelper.sol";

import "contracts/EphemeralGetPopulatedTicksInRangePCSV4.sol";
import "./BasePCSV4.t.sol";

contract TickLensPCSV4Test is BasePCSV4Test, PoolUtilsPCSV4 {
    function verifyTicks(PopulatedTick[] memory populatedTicks) internal view {
        for (uint256 i; i < populatedTicks.length; ++i) {
            PopulatedTick memory populatedTick = populatedTicks[i];
            Tick.Info memory tickInfo =
                ICLPoolManager(poolManagerAddr).getPoolTickInfo(poolKey.toId(), populatedTick.tick);
            assertEq(tickInfo.liquidityGross, populatedTick.liquidityGross, "liquidityGross");
            assertEq(tickInfo.liquidityNet, populatedTick.liquidityNet, "liquidityNet");
            assertEq(tickInfo.feeGrowthOutside0X128, populatedTick.feeGrowthOutside0X128, "feeGrowthOutside0X128");
            assertEq(tickInfo.feeGrowthOutside1X128, populatedTick.feeGrowthOutside1X128, "feeGrowthOutside1X128");
        }
    }

    function test_GetPopulatedTicksInRangePCSV4() public {
        int24 tick = currentTick();
        int24 width = 10;
        int24 tickSpacing = CLPoolParametersHelper.getTickSpacing(poolKey.parameters);
        try new EphemeralGetPopulatedTicksInRangePCSV4(
            poolManagerAddr, poolKey, tick - width * tickSpacing, tick + width * tickSpacing
        ) {}
        catch (bytes memory returnData) {
            PopulatedTick[] memory populatedTicks = abi.decode(returnData, (PopulatedTick[]));
            verifyTicks(populatedTicks);
        }
    }
}
