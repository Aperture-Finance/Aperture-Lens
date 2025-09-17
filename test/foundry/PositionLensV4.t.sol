// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {IPoolManager} from "@uniswap/v4-core/src/interfaces/IPoolManager.sol";
import {IPositionManager} from "@uniswap/v4-periphery/src/interfaces/IPositionManager.sol";
import {PositionInfo, PositionInfoLibrary} from "@uniswap/v4-periphery/src/libraries/PositionInfoLibrary.sol";
// import {PositionState} from "contracts/PositionUtilsV4.sol";
import "contracts/EphemeralGetPositionV4.sol";
import "contracts/EphemeralGetPositionsV4.sol";
import "contracts/PositionLensV4.sol";
import "./BaseV4.t.sol";

contract PositionLensV4Test is BaseV4Test {
    PositionLensV4 internal positionLens;
    int24 internal constant tickLower = -192460;
    int24 internal constant tickUpper = -191210;
    // The owner should be position manager from pool manager's perspective.
    address internal constant owner = posManagerAddr;
    address internal constant erc721Owner = 0xbeef63AE5a2102506e8a352a5bB32aA8B30B3112;
    uint256 internal constant tokenId = 60390;
    bytes32 internal constant salt = bytes32(tokenId);
    IPoolManager poolM = IPoolManager(poolManagerAddr);
    IPositionManager posM = IPositionManager(posManagerAddr);

    function setUp() public virtual override {
        super.setUp();
        positionLens = new PositionLensV4();
    }

    function test_GetFeesOwed() public view {
        (uint256 fees0, uint256 fees1) = positionLens.getFeesOwed(poolM, poolKey, owner, tickLower, tickUpper, salt);
        assertEq(fees0, 148410618806);
        assertEq(fees1, 1733);
    }

    function test_GetTotalAmounts() public view {
        (uint256 amount0, uint256 amount1) =
            positionLens.getTotalAmounts(poolM, poolKey, owner, tickLower, tickUpper, salt);
        assertEq(amount0, 24907907374597336);
        assertEq(amount1, 37180040);
    }

    function verifyPosition(PositionState memory state) internal view virtual {
        assertEq(state.owner, erc721Owner);
        assertEq(state.tokenId, tokenId);
        assertEq(Currency.unwrap(state.poolKey.currency0), address(0));
        assertEq(Currency.unwrap(state.poolKey.currency1), USDC);
        assertEq(state.activeLiquidity, 1141661974404565126);
        assertEq(state.slot0.sqrtPriceX96, 5327567843637637731281362);
        assertEq(state.decimals0, 18);
        assertEq(state.decimals1, 6);

        PositionFull memory position = state.position;
        assertEq(position.operator, address(0), "operator");
        assertEq(position.token0, address(0), "token0");
        assertEq(position.token1, USDC, "token1");
        assertEq(position.feeOrTickSpacing, 500, "feeOrTickSpacing"); // 0.5% fee tier
        assertEq(position.tickLower, tickLower, "tickLower");
        assertEq(position.tickUpper, tickUpper, "tickUpper");
        assertEq(position.liquidity, 36351160423339, "position's liquidity");
        assertEq(
            position.feeGrowthInside0LastX128, 1672432573071112639701363892899143683507, "feeGrowthInside0LastX128"
        );
        assertEq(position.feeGrowthInside1LastX128, 7765894095507657306717359494486, "feeGrowthInside1LastX128");
        assertEq(position.tokensOwed0, 148410618806, "tokensOwed0");
        assertEq(position.tokensOwed1, 1733, "tokensOwed1");
        assertEq(position.salt, salt, "salt");
    }

    function test_GetPosition() public virtual {
        try new EphemeralGetPositionV4(posManagerAddr, tokenId) {}
        catch (bytes memory returnData) {
            PositionState memory state = abi.decode(returnData, (PositionState));
            verifyPosition(state);
        }
    }

    function test_GetPositions() public virtual {
        uint256[] memory tokenIds = new uint256[](2);
        tokenIds[0] = tokenId;
        tokenIds[1] = tokenId;
        try new EphemeralGetPositionsV4(posManagerAddr, tokenIds) {}
        catch (bytes memory returnData) {
            PositionState[] memory states = abi.decode(returnData, (PositionState[]));
            assertEq(states.length, 2, "mismatch length");
            for (uint256 i = 0; i < states.length; ++i) {
                verifyPosition(states[i]);
            }
        }
    }
}
