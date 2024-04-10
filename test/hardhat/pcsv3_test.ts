import { TickMath } from "@uniswap/v3-sdk";
import { expect } from "chai";
import { ContractFunctionReturnType, createPublicClient, getContract, http, toHex } from "viem";
import { bsc } from "viem/chains";
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
  INonfungiblePositionManager__factory,
  IPancakeV3Pool__factory,
} from "../../typechain";
import { computePoolAddress } from "@pancakeswap/v3-sdk";
import { Token } from "@pancakeswap/sdk";

const AMM = AutomatedMarketMakerEnum.enum.PANCAKESWAP_V3;
const PCSV3_NPM = "0x46A15B0b27311cedF172AB29E4f4766fbE7F4364";
const PCSV3_POOL_DEPLOYER = "0x41ff9AA7e16B8B1a8a8dc4f0eFacd93D02d071c9";
const USDT_ADDRESS = "0x55d398326f99059fF775485246999027B3197955";
const WBNB_ADDRESS = "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c";

describe("Pool lens test with PCSV3 on BSC", () => {
  const publicClient = createPublicClient({
    chain: bsc,
    transport: http(`https://bsc-rpc.publicnode.com`),
    batch: {
      multicall: true,
    },
  });
  var blockNumber: bigint;
  const pool = computePoolAddress({
    deployerAddress: PCSV3_POOL_DEPLOYER,
    tokenA: new Token(bsc.id, USDT_ADDRESS, 6, "USDT"),
    tokenB: new Token(bsc.id, WBNB_ADDRESS, 18, "WBNB"),
    fee: 500,
  });
  const poolContract = getContract({
    address: pool,
    abi: IPancakeV3Pool__factory.abi,
    client: publicClient,
  });
  const npm = getContract({
    address: PCSV3_NPM,
    abi: INonfungiblePositionManager__factory.abi,
    client: publicClient,
  });

  before(async () => {
    blockNumber = (await publicClient.getBlockNumber()) - 64n;
    console.log(`Running PCSV3 tests on the BNB chain at block number ${blockNumber}...`);
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
      position: { token0, token1, fee },
      slot0: { sqrtPriceX96, tick },
    } = await getPositionDetails(PCSV3_NPM, 4n, publicClient, blockNumber);
    expect(tokenId).to.be.eq(4n);
    const poolAddress = computePoolAddress({
      deployerAddress: PCSV3_POOL_DEPLOYER,
      tokenA: new Token(bsc.id, token0, 0, "NOT_USED"),
      tokenB: new Token(bsc.id, token1, 0, "NOT_USED"),
      fee,
    });
    const [_sqrtPriceX96, _tick] = await getContract({
      address: poolAddress,
      abi: IPancakeV3Pool__factory.abi,
      client: publicClient,
    }).read.slot0({ blockNumber });
    expect(sqrtPriceX96).to.be.eq(_sqrtPriceX96);
    expect(tick).to.be.eq(_tick);
  });

  async function verifyPositionDetails(posArr: ContractFunctionReturnType<typeof EphemeralGetPositions__factory.abi>) {
    await Promise.all(
      posArr.map(async ({ tokenId, position }) => {
        const [, , token0, token1, fee, tickLower, tickUpper, liquidity] = await npm.read.positions([tokenId], {
          blockNumber,
        });
        expect(position.token0).to.be.eq(token0);
        expect(position.token1).to.be.eq(token1);
        expect(position.fee).to.be.eq(fee);
        expect(position.tickLower).to.be.eq(tickLower);
        expect(position.tickUpper).to.be.eq(tickUpper);
        expect(position.liquidity).to.be.eq(liquidity);
      }),
    );
  }

  it("Test getting position array", async () => {
    const posArr = await getPositions(
      PCSV3_NPM,
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
    const posArr = await getAllPositionsByOwner(PCSV3_NPM, owner, publicClient, blockNumber);
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
        fromBlock: blockNumber - 10000n,
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
