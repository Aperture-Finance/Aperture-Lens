import { zeroAddress } from "viem";
import { expect } from "chai";
import { ContractFunctionReturnType, createPublicClient, getContract, http } from "viem";
import {
  getPopulatedTicksInRangeV4,
  getPositionDetailsV4,
  getPositionsV4,
  PoolKeyStruct,
} from "../../src/viem";
import {
  EphemeralGetPositionsV4__factory,
  IPositionManager__factory,
  IStateView__factory,
} from "../../typechain";
import { mainnet } from "viem/chains";
import { Token } from "@uniswap/sdk-core";
import { Pool } from '@uniswap/v4-sdk';

const UNIV4_POOL_MANAGER = "0x000000000004444c5dc75cB358380D2e3dE08A90";
const UNIV4_POSITION_MANAGER = "0xbD216513d74C8cf14cf4747E6AaA6420FF64ee9e";
const UNIV4_STATE_VIEW = "0x7fFE42C4a5DEeA5b0feC41C94C136Cf115597227";
const USDC_ADDRESS = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";

describe("Pool lens test with UniV4 on mainnet", () => {
  const publicClient = createPublicClient({
    chain: mainnet,
    transport: http("https://ethereum-rpc.publicnode.com"),
    batch: {
      multicall: true,
    },
  });
  let blockNumber: bigint;
  const stateView = getContract({
    address: UNIV4_STATE_VIEW,
    abi: IStateView__factory.abi,
    client: publicClient,
  });
  const positionManager = getContract({
    address: UNIV4_POSITION_MANAGER,
    abi: IPositionManager__factory.abi,
    client: publicClient,
  });
  const TOKEN_ID: bigint = 60390n;
  const poolKey: PoolKeyStruct = {
    currency0: zeroAddress,
    currency1: USDC_ADDRESS,
    fee: 500,
    tickSpacing: 10,
    hooks: zeroAddress
  };
  const ETH_TOKEN = new Token(
    1,
    zeroAddress,
    18,
    'ETH',
    'Ether'
  )

  const USDC_TOKEN = new Token(
    1,
    USDC_ADDRESS,
    6,
    'USDC',
    'USDC'
  )
  const poolId = Pool.getPoolId(ETH_TOKEN, USDC_TOKEN, 500, 10, zeroAddress) as `0x${string}`;

  before(async () => {
    blockNumber = (await publicClient.getBlockNumber()) - 64n;
    console.log(`Running UniV4 tests on Ethereum mainnet at block number ${blockNumber}...`);
  });

  it("Test getting populated ticks", async () => {
    const [, tickCurrent, ,] = await stateView.read.getSlot0([poolId], { blockNumber });
    console.log("Inside hardhat test");
    console.log("tickCurrent: ", tickCurrent);
    const ticks = await getPopulatedTicksInRangeV4(UNIV4_POOL_MANAGER, poolKey, tickCurrent, tickCurrent, publicClient, blockNumber);
    await Promise.all(
      ticks.map(async ({ tick, liquidityGross, liquidityNet, feeGrowthOutside0X128, feeGrowthOutside1X128 }) => {
        const [_liquidityGross, _liquidityNet, _feeGrowthOutside0X128, _feeGrowthOutside1X128] = await stateView.read.getTickInfo([poolId, tick], { blockNumber });
        expect(liquidityGross).to.be.eq(_liquidityGross);
        expect(liquidityNet).to.be.eq(_liquidityNet);
        expect(feeGrowthOutside0X128).to.be.eq(_feeGrowthOutside0X128);
        expect(feeGrowthOutside1X128).to.be.eq(_feeGrowthOutside1X128);
      }),
    );
  });

  it("Test getting single position details", async () => {
    const {
      tokenId,
      slot0: { sqrtPriceX96, tick },
    } = await getPositionDetailsV4(UNIV4_POSITION_MANAGER, TOKEN_ID, publicClient, blockNumber);
    expect(tokenId).to.be.eq(TOKEN_ID);
    const [_sqrtPriceX96, _tick, ,] = await stateView.read.getSlot0([poolId], { blockNumber });
    expect(sqrtPriceX96).to.be.eq(_sqrtPriceX96);
    expect(tick).to.be.eq(_tick);
  });

  it("Test getting position array", async () => {
    const positions = await getPositionsV4(
      UNIV4_POSITION_MANAGER, [TOKEN_ID, TOKEN_ID],
      publicClient,
      blockNumber,
    );
    await verifyPositionDetails(positions);
  });

  async function verifyPositionDetails(positions: ContractFunctionReturnType<typeof EphemeralGetPositionsV4__factory.abi>) {
    await Promise.all(
      positions.map(async ({ tokenId, position, poolFee }) => {
        const liquidity = await positionManager.read.getPositionLiquidity([TOKEN_ID], {
          blockNumber,
        });
        expect(position.liquidity).to.be.eq(liquidity);
      }),
    );
  }

});
