import { ApertureSupportedChainId, getChainInfo, viem } from "@aperture_finance/uniswap-v3-automation-sdk";
import { TickMath } from "@uniswap/v3-sdk";
import { expect } from "chai";
import { config as dotenvConfig } from "dotenv";
import { ContractFunctionReturnType, createPublicClient, getContract, http, toHex } from "viem";
import {
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
import { EphemeralGetPositions__factory, EphemeralPoolSlots__factory, IUniswapV3Pool__factory } from "../../typechain";

dotenvConfig();

const chainId = ApertureSupportedChainId.ETHEREUM_MAINNET_CHAIN_ID;
const USDC_ADDRESS = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("Pool lens test with UniV3 on mainnet", () => {
  const { chain, uniswap_v3_factory, uniswap_v3_nonfungible_position_manager } = getChainInfo(chainId);
  const publicClient = createPublicClient({
    chain,
    transport: http("https://ethereum-rpc.publicnode.com"),
    batch: {
      multicall: true,
    },
  });
  let blockNumber: bigint;
  const pool = viem.computePoolAddress(uniswap_v3_factory, USDC_ADDRESS, WETH_ADDRESS, 500);
  const poolContract = getContract({
    address: pool,
    abi: IUniswapV3Pool__factory.abi,
    client: publicClient,
  });
  const npm = viem.getNPM(chainId, publicClient);

  before(async () => {
    blockNumber = await publicClient.getBlockNumber();
    console.log(`Running UniV3 tests on Ethereum mainnet at block number ${blockNumber}...`);
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
    } = await getPositionDetails(uniswap_v3_nonfungible_position_manager, 4n, publicClient, blockNumber);
    expect(tokenId).to.be.eq(4n);
    const [_sqrtPriceX96, _tick] = await getContract({
      address: viem.computePoolAddress(uniswap_v3_factory, token0, token1, fee),
      abi: IUniswapV3Pool__factory.abi,
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
      uniswap_v3_nonfungible_position_manager,
      Array.from({ length: 100 }, (_, i) => BigInt(i + 1)),
      publicClient,
      blockNumber,
    );
    await verifyPositionDetails(posArr);
  });

  it("Test getting all positions by owner", async () => {
    const totalSupply = await npm.read.totalSupply({ blockNumber });
    const owner = await npm.read.ownerOf([totalSupply - 1n], { blockNumber });
    const posArr = await getAllPositionsByOwner(
      uniswap_v3_nonfungible_position_manager,
      owner,
      publicClient,
      blockNumber,
    );
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
    const slots = await getStaticSlots(pool, publicClient, blockNumber);
    await verifySlots(slots);
  });

  it("Test getting populated ticks slots", async () => {
    const slots = await getTicksSlots(pool, TickMath.MIN_TICK, TickMath.MAX_TICK, publicClient, blockNumber);
    await verifySlots(slots);
  });

  it("Test getting tick bitmap slots", async () => {
    const slots = await getTickBitmapSlots(pool, publicClient, blockNumber);
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
    const slots = await getPositionsSlots(pool, positions, publicClient, blockNumber);
    await verifySlots(slots);
  });
});
