// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IPCSV3NonfungiblePositionManager} from "@aperture_finance/uni-v3-lib/src/interfaces/INonfungiblePositionManager.sol";
import {PoolAddress} from "@aperture_finance/uni-v3-lib/src/PoolAddress.sol";
import {PoolAddressPancakeSwapV3} from "@aperture_finance/uni-v3-lib/src/PoolAddressPancakeSwapV3.sol";
import {IPancakeV3Pool} from "@pancakeswap/v3-core/contracts/interfaces/IPancakeV3Pool.sol";
import "contracts/EphemeralAllPositionsByOwner.sol";
import "contracts/EphemeralGetPosition.sol";
import "contracts/EphemeralGetPositions.sol";
import "contracts/PositionLens.sol";
import "./Base.t.sol";

contract PositionLensTest is BaseTest {
    PositionLens internal positionLens;
    int24 internal _tickLower;
    int24 internal _tickUpper;
    uint256 internal lastTokenId;

    function setUp() public virtual override {
        super.setUp();
        lastTokenId = npm.totalSupply();
        positionLens = new PositionLens();
        int24 tick = currentTick();
        _tickLower = tick - tickSpacing;
        _tickUpper = tick + tickSpacing;
        (_tickLower, _tickUpper) = prepTicks(_tickLower, _tickUpper);
        uint256 amount0Desired = token0Unit * 1000;
        uint256 amount1Desired = token1Unit * 1000;
        deal(token0, address(this), amount0Desired);
        deal(token1, address(this), amount1Desired);
        mint(address(this), amount0Desired, amount1Desired, _tickLower, _tickUpper);
    }

    function generateFees() internal {
        uint160 initialPrice = sqrtPriceX96();
        uint256 amountIn = 1000 * token0Unit;
        deal(token0, address(this), 2 * amountIn);
        deal(token1, address(this), 1000 * token1Unit);
        for (uint256 i; i < 10; ++i) {
            swapBackAndForth(amountIn, true, initialPrice);
        }
    }

    function test_GetFeesOwed() public {
        (uint256 fees0, uint256 fees1) = positionLens.getFeesOwed(
            V3PoolCallee.wrap(pool),
            address(this),
            _tickLower,
            _tickUpper
        );
        assertEq(fees0, 0);
        assertEq(fees1, 0);
        generateFees();
        (fees0, fees1) = positionLens.getFeesOwed(V3PoolCallee.wrap(pool), address(this), _tickLower, _tickUpper);
        IUniswapV3Pool(pool).burn(_tickLower, _tickUpper, 0);
        (uint128 amount0, uint128 amount1) = IUniswapV3Pool(pool).collect(
            address(this),
            _tickLower,
            _tickUpper,
            type(uint128).max,
            type(uint128).max
        );
        assertEq(fees0, amount0);
        assertEq(fees1, amount1);
    }

    function test_GetTotalAmounts() public {
        generateFees();
        (uint256 amount0, uint256 amount1) = positionLens.getTotalAmounts(
            V3PoolCallee.wrap(pool),
            address(this),
            _tickLower,
            _tickUpper
        );
        PoolCaller.PositionInfo memory info = V3PoolCallee.wrap(pool).positions(
            keccak256(abi.encodePacked(address(this), _tickLower, _tickUpper))
        );
        IUniswapV3Pool(pool).burn(_tickLower, _tickUpper, info.liquidity);
        (uint128 collect0, uint128 collect1) = IUniswapV3Pool(pool).collect(
            address(this),
            _tickLower,
            _tickUpper,
            type(uint128).max,
            type(uint128).max
        );
        assertEq(collect0, amount0);
        assertEq(collect1, amount1);
    }

    function verifyPosition(PositionState memory pos) internal {
        {
            assertEq(pos.owner, npm.ownerOf(pos.tokenId), "owner");
            (, , address token0, , uint24 fee, int24 tickLower, , uint128 liquidity, , , , ) = npm.positions(
                pos.tokenId
            );
            assertEq(token0, pos.position.token0, "token0");
            assertEq(fee, pos.position.fee, "fee");
            assertEq(tickLower, pos.position.tickLower, "tickLower");
            assertEq(liquidity, pos.position.liquidity, "liquidity");
        }
        {
            address pool = IUniswapV3Factory(factory).getPool(
                pos.position.token0,
                pos.position.token1,
                pos.position.fee
            );
            (
                uint160 sqrtPriceX96,
                int24 tick,
                uint16 observationIndex,
                uint16 observationCardinality,
                uint16 observationCardinalityNext,
                uint32 feeProtocol,
                bool unlocked
            ) = IPancakeV3Pool(pool).slot0();
            assertEq(sqrtPriceX96, pos.slot0.sqrtPriceX96, "sqrtPriceX96");
            assertEq(tick, pos.slot0.tick, "tick");
            assertEq(observationIndex, pos.slot0.observationIndex, "observationIndex");
            assertEq(observationCardinality, pos.slot0.observationCardinality, "observationCardinality");
            assertEq(observationCardinalityNext, pos.slot0.observationCardinalityNext, "observationCardinalityNext");
            assertEq(feeProtocol, pos.slot0.feeProtocol, "feeProtocol");
            assertEq(unlocked, pos.slot0.unlocked, "unlocked");
            assertEq(IUniswapV3Pool(pool).liquidity(), pos.activeLiquidity, "liquidity");
        }
        assertEq(IERC20Metadata(pos.position.token0).decimals(), pos.decimals0, "decimals0");
        assertEq(IERC20Metadata(pos.position.token1).decimals(), pos.decimals1, "decimals1");
    }

    /// forge-config: default.fuzz.runs = 16
    /// forge-config: ci.fuzz.runs = 16
    function testFuzz_GetPosition(uint256 tokenId) public virtual {
        tokenId = bound(tokenId, 1, 200);
        try new EphemeralGetPosition(npm, tokenId) {} catch (bytes memory returnData) {
            PositionState memory pos = abi.decode(returnData, (PositionState));
            verifyPosition(pos);
        }
    }

    function test_GetPositions() public virtual {
        uint256 startTokenId = 100;
        uint256[] memory tokenIds = new uint256[](10);
        for (uint256 i; i < 10; ++i) {
            tokenIds[i] = startTokenId + i;
        }
        try new EphemeralGetPositions(npm, tokenIds) {} catch (bytes memory returnData) {
            PositionState[] memory positions = abi.decode(returnData, (PositionState[]));
            uint256 length = positions.length;
            console2.log("length", length);
            for (uint256 i; i < length; ++i) {
                verifyPosition(positions[i]);
            }
        }
    }

    function test_AllPositions() public {
        address owner = npm.ownerOf(lastTokenId);
        try new EphemeralAllPositionsByOwner(npm, owner) {} catch (bytes memory returnData) {
            PositionState[] memory positions = abi.decode(returnData, (PositionState[]));
            uint256 length = positions.length;
            console2.log("balance", length);
            for (uint256 i; i < length; ++i) {
                verifyPosition(positions[i]);
            }
        }
    }
}

contract PCSV3PositionLensTest is PositionLensTest {
    function setUp() public override {
        dex = DEX.PancakeSwapV3;
        super.setUp();
    }

    // Trivially override so that the "forge-config" settings are applied.
    /// forge-config: default.fuzz.runs = 16
    /// forge-config: ci.fuzz.runs = 16
    function testFuzz_GetPosition(uint256 tokenId) public override {
        super.testFuzz_GetPosition(tokenId);
    }
}
