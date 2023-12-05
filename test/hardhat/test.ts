import { ApertureSupportedChainId, getChainInfo, viem } from "@aperture_finance/uniswap-v3-automation-sdk";
import { TickMath } from "@uniswap/v3-sdk";
import { expect } from "chai";
import { config as dotenvConfig } from "dotenv";
import { createPublicClient, getContract, http, toHex } from "viem";
import { getTicksSlots, getPositionsSlots, getStaticSlots, getTickBitmapSlots } from "../../src/viem/poolLens";
import { IUniswapV3Pool__factory } from "../../typechain";

dotenvConfig();

const chainId = ApertureSupportedChainId.ETHEREUM_MAINNET_CHAIN_ID;
const USDC_ADDRESS = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("Pool lens test", () => {
  const { chain, uniswap_v3_factory } = getChainInfo(chainId);
  const publicClient = createPublicClient({
    chain,
    transport: http(`https://mainnet.infura.io/v3/${process.env.INFURA_API_KEY}`),
    batch: {
      multicall: true,
    },
  });
  const blockNumber = 17000000n;
  const pool = viem.computePoolAddress(uniswap_v3_factory, USDC_ADDRESS, WETH_ADDRESS, 500);
  const poolContract = getContract({
    address: pool,
    abi: IUniswapV3Pool__factory.abi,
    publicClient,
  });

  it("Test getting static storage slots", async () => {
    const slots = await getStaticSlots(pool, publicClient, blockNumber);
    expect(slots.some(({ data }) => data > 0)).to.be.true;
    const address = pool;
    const altSlots = await Promise.all([
      publicClient.getStorageAt({ address, slot: toHex(0), blockNumber }),
      publicClient.getStorageAt({ address, slot: toHex(1), blockNumber }),
      publicClient.getStorageAt({ address, slot: toHex(2), blockNumber }),
      publicClient.getStorageAt({ address, slot: toHex(3), blockNumber }),
    ]);
    for (let i = 0; i < 4; i++) {
      expect(slots[i].data).to.be.eq(BigInt(altSlots[i]!));
    }
  });

  it("Test getting populated ticks slots", async () => {
    const slots = await getTicksSlots(pool, TickMath.MIN_TICK, TickMath.MAX_TICK, publicClient, blockNumber);
    expect(slots.some(({ data }) => data > 0)).to.be.true;
  });

  it("Test getting tick bitmap slots", async () => {
    const slots = await getTickBitmapSlots(pool, publicClient, blockNumber);
    expect(slots.some(({ data }) => data > 0)).to.be.true;
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
    expect(slots.some(({ data }) => data > 0)).to.be.true;
  });
});
