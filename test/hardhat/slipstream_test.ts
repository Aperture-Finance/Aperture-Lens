import { TickMath } from "@uniswap/v3-sdk";
import { expect } from "chai";
import { Address, ContractFunctionReturnType, createPublicClient, getContract, http, toHex } from "viem";
import {
  AutomatedMarketMakerEnum,
  getAllPositionsByOwner,
  getPopulatedTicksInRange,
  getPositionDetails,
  getPositions,
  getPositionsSlots,
  getStaticSlots,
  getStorageAt,
  getTickBitmapSlots,
  getTicksSlots,
} from "../../src/viem";
import {
  EphemeralGetPositions__factory,
  EphemeralPoolSlots__factory,
  ISlipStreamCLFactory__factory,
  ISlipStreamNonfungiblePositionManager__factory,
} from "../../typechain";
import { base } from "viem/chains";
import 'dotenv/config';
import SlipStreamPoolAbi from './SlipStreamPool_abi.json';

const AMM = AutomatedMarketMakerEnum.enum.SLIPSTREAM;
const SLIPSTREAM_NPM = "0x827922686190790b37229fd06084350E74485b72";
const SLIPSTREAM_FACTORY = "0x5e7BB104d84c7CB9B682AaC2F3d509f5F406809A";
const USDC_ADDRESS = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const WETH_ADDRESS = "0x4200000000000000000000000000000000000006";

describe("Pool lens test with SlipStream on Base", () => {
  const publicClient = createPublicClient({
    chain: base,
    transport: http(`${process.env.BASE_RPC_URL}`),
    batch: {
      multicall: true,
    },
  });
  const factoryContract = getContract({
    address: SLIPSTREAM_FACTORY,
    abi: ISlipStreamCLFactory__factory.abi,
    client: publicClient,
  });
  const npm = getContract({
    address: SLIPSTREAM_NPM,
    abi: ISlipStreamNonfungiblePositionManager__factory.abi,
    client: publicClient,
  });
  let blockNumber: bigint;
  let pool: Address;
  let poolContract;

  before(async () => {
    blockNumber = (await publicClient.getBlockNumber()) - 64n;
    console.log(`Running SlipStream tests on Base mainnet at block number ${blockNumber}...`);
    pool = await factoryContract.read.getPool([WETH_ADDRESS, USDC_ADDRESS, /*tickSpacing=*/100], { blockNumber });
    poolContract = getContract({
      abi: SlipStreamPoolAbi,
      client: publicClient,
      address: pool,
    });
  });

  it("Test extsload", async () => {
    const slots = await getStorageAt(
      pool,
      Array.from({ length: 4 }, (_, i) => toHex(i, { size: 32 })),
      publicClient,
      blockNumber,
    );
    await Promise.all(
      slots.map(async (slot, i) => {
        const _slot = await publicClient.getStorageAt({ address: pool, slot: toHex(i), blockNumber });
        expect(slot).to.be.eq(_slot);
      }),
    );
  });

  it("Test getting populated ticks", async () => {
    const [, tickCurrent] = await poolContract.read.slot0({ blockNumber });
    const ticks = await getPopulatedTicksInRange(pool, tickCurrent, tickCurrent, publicClient, blockNumber);
    await Promise.all(
      ticks.map(async ({ tick, liquidityGross, liquidityNet }) => {
        const [_liquidityGross, _liquidityNet] = await poolContract.read.ticks([tick], { blockNumber });
        expect(liquidityGross).to.be.eq(_liquidityGross);
        expect(liquidityNet).to.be.eq(_liquidityNet);
      }),
    );
  });

  it("Test getting position details", async () => {
    const {
      tokenId,
      position: { token0, token1, feeOrTickSpacing: tickSpacing },
      slot0: { sqrtPriceX96, tick },
    } = await getPositionDetails(AMM, SLIPSTREAM_NPM, 43484n, publicClient, blockNumber);
    expect(tokenId).to.be.eq(43484n);
    const [_sqrtPriceX96, _tick] = await getContract({
      address: await factoryContract.read.getPool([token0, token1, tickSpacing], { blockNumber }),
      abi: SlipStreamPoolAbi,
      client: publicClient,
    }).read.slot0({ blockNumber });
    expect(sqrtPriceX96).to.be.eq(_sqrtPriceX96);
    expect(tick).to.be.eq(_tick);
  });

  async function verifyPositionDetails(posArr: ContractFunctionReturnType<typeof EphemeralGetPositions__factory.abi>) {
    await Promise.all(
      posArr.map(async ({ tokenId, position, poolTickSpacing }) => {
        const [, , token0, token1, tickSpacing, tickLower, tickUpper, liquidity] = await npm.read.positions([tokenId], {
          blockNumber,
        });
        expect(position.token0).to.be.eq(token0);
        expect(position.token1).to.be.eq(token1);
        expect(position.feeOrTickSpacing).to.be.eq(tickSpacing);
        expect(position.tickLower).to.be.eq(tickLower);
        expect(position.tickUpper).to.be.eq(tickUpper);
        expect(position.liquidity).to.be.eq(liquidity);
        expect(poolTickSpacing).to.be.eq(tickSpacing);
      }),
    );
  }

  it("Test getting position array", async () => {
    const posArr = await getPositions(
      AMM,
      SLIPSTREAM_NPM,
      Array.from({ length: 100 }, (_, i) => BigInt(i + 1)),
      publicClient,
      blockNumber,
    );
    await verifyPositionDetails(posArr);
  });

  it("Test getting all positions by owner", async () => {
    const totalSupply = await npm.read.totalSupply({ blockNumber });
    const tokenId = await npm.read.tokenByIndex([totalSupply - 1n], { blockNumber });
    const owner = await npm.read.ownerOf([tokenId], { blockNumber });
    const posArr = await getAllPositionsByOwner(AMM, SLIPSTREAM_NPM, owner, publicClient, blockNumber);
    await verifyPositionDetails(posArr);
  });

  async function verifySlots(slots: ContractFunctionReturnType<typeof EphemeralPoolSlots__factory.abi>) {
    expect(slots.some(({ data }) => data > 0)).to.be.true;
    const address = pool;
    const altSlots = await Promise.all(
      slots.slice(0, 4).map(({ slot }) => publicClient.getStorageAt({ address, slot: toHex(slot), blockNumber })),
    );
    for (let i = 0; i < altSlots.length; i++) {
      expect(slots[i].data).to.be.eq(BigInt(altSlots[i]!));
    }
  }

  it("Test getting static storage slots", async () => {
    const slots = await getStaticSlots(AMM, pool, publicClient, blockNumber);
    await verifySlots(slots);
  });

  it("Test getting populated ticks slots", async () => {
    const slots = await getTicksSlots(AMM, pool, TickMath.MIN_TICK, TickMath.MAX_TICK, publicClient, blockNumber);
    await verifySlots(slots);
  });

  it("Test getting tick bitmap slots", async () => {
    const slots = await getTickBitmapSlots(AMM, pool, publicClient, blockNumber);
    await verifySlots(slots);
  });

  it("Test getting positions mapping slots", async () => {
    const logs = await poolContract.getEvents.Mint(
      {},
      {
        fromBlock: blockNumber - 1000n,
        toBlock: blockNumber,
      },
    );
    const positions = logs.map(({ args: { owner, tickLower, tickUpper } }) => ({
      owner: owner!,
      tickLower: tickLower!,
      tickUpper: tickUpper!,
    }));
    const slots = await getPositionsSlots(AMM, pool, positions, publicClient, blockNumber);
    await verifySlots(slots);
  });
});
