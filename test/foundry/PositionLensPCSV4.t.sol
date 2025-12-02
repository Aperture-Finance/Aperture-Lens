// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {ICLPoolManager} from "infinity-core/src/pool-cl/interfaces/ICLPoolManager.sol";
import {ICLPositionManager} from "infinity-periphery/src/pool-cl/interfaces/ICLPositionManager.sol";
import {
    CLPositionInfo,
    CLPositionInfoLibrary
} from "infinity-periphery/src/pool-cl/libraries/CLPositionInfoLibrary.sol";

import "contracts/EphemeralGetPositionPCSV4.sol";
import "contracts/EphemeralGetPositionsPCSV4.sol";
import "contracts/PositionLensPCSV4.sol";
import "./BasePCSV4.t.sol";

contract PositionLensPCSV4Test is BasePCSV4Test {
    PositionLensPCSV4 internal positionLens;
    int24 internal constant tickLower = 66260;
    int24 internal constant tickUpper = 71080;
    // The owner should be position manager from pool manager's perspective.
    address internal constant owner = posManagerAddr;
    address internal constant erc721Owner = 0xbeef63AE5a2102506e8a352a5bB32aA8B30B3112;
    uint256 internal constant tokenId = 156677;
    bytes32 internal constant salt = bytes32(tokenId);
    ICLPoolManager poolM = ICLPoolManager(poolManagerAddr);
    ICLPositionManager posM = ICLPositionManager(posManagerAddr);

    function setUp() public virtual override {
        super.setUp();
        positionLens = new PositionLensPCSV4();
    }

    function test_GetFeesOwed() public view {
        (uint256 fees0, uint256 fees1) = positionLens.getFeesOwed(poolM, poolKey, owner, tickLower, tickUpper, salt);
        assertEq(fees0, 85809316870);
        assertEq(fees1, 66949868027046);
    }

    function test_GetTotalAmounts() public view {
        (uint256 amount0, uint256 amount1) =
            positionLens.getTotalAmounts(poolM, poolKey, owner, tickLower, tickUpper, salt);
        assertEq(amount0, 76154253555335);
        assertEq(amount1, 16400602644020840);
    }

    function verifyPosition(PositionState memory state) internal view virtual {
        assertEq(state.poolKey.tickSpacing, 10, "tickSpacing");
        assertEq(state.owner, erc721Owner);
        assertEq(state.tokenId, tokenId);
        assertEq(state.activeLiquidity, 5136408834887709916624);
        assertEq(state.slot0.sqrtPriceX96, 2280147888752920416539891832455);
        assertEq(state.decimals0, 18);
        assertEq(state.decimals1, 18);

        PositionFull memory position = state.position;
        assertEq(position.operator, address(0), "operator");
        assertEq(position.token0, address(0), "token0");
        assertEq(position.token1, USDC, "token1");
        assertEq(position.feeOrTickSpacing, 8388608, "feeOrTickSpacing"); // dynamic fee tier
        assertEq(position.tickLower, tickLower, "tickLower");
        assertEq(position.tickUpper, tickUpper, "tickUpper");
        assertEq(position.liquidity, 12405391974192267, "position's liquidity");
        assertEq(position.feeGrowthInside0LastX128, 0, "feeGrowthInside0LastX128");
        assertEq(position.feeGrowthInside1LastX128, 0, "feeGrowthInside1LastX128");
        assertEq(position.tokensOwed0, 85809316870, "tokensOwed0");
        assertEq(position.tokensOwed1, 66949868027046, "tokensOwed1");
        assertEq(position.salt, salt, "salt");
    }

    function test_GetPosition() public virtual {
        try new EphemeralGetPositionPCSV4(posManagerAddr, tokenId) {}
        catch (bytes memory returnData) {
            PositionState memory state = abi.decode(returnData, (PositionState));
            verifyPosition(state);
        }
    }

    function test_GetPositions() public virtual {
        uint256[] memory tokenIds = new uint256[](2);
        tokenIds[0] = tokenId;
        tokenIds[1] = tokenId;
        try new EphemeralGetPositionsPCSV4(posManagerAddr, tokenIds) {}
        catch (bytes memory returnData) {
            PositionState[] memory states = abi.decode(returnData, (PositionState[]));
            assertEq(states.length, 2, "mismatch length");
            for (uint256 i = 0; i < states.length; ++i) {
                verifyPosition(states[i]);
            }
        }
    }
}
