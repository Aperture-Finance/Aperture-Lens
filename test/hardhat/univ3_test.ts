import { TickMath, computePoolAddress } from "@uniswap/v3-sdk";
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
  IUniswapV3NonfungiblePositionManager__factory,
  IUniswapV3Pool__factory,
} from "../../typechain";
import { mainnet } from "viem/chains";
import { Token } from "@uniswap/sdk-core";

const AMM = AutomatedMarketMakerEnum.enum.UNISWAP_V3;
const UNIV3_NPM = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
const UNIV3_FACTORY = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
const USDC_ADDRESS = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

describe("Pool lens test with UniV3 on mainnet", () => {
  const publicClient = createPublicClient({
    chain: mainnet,
    transport: http("https://ethereum-rpc.publicnode.com"),
    batch: {
      multicall: true,
    },
  });
  let blockNumber: bigint;
  const pool = computePoolAddress({
    factoryAddress: UNIV3_FACTORY,
    tokenA: new Token(mainnet.id, USDC_ADDRESS, 6, "USDC"),
    tokenB: new Token(mainnet.id, WETH_ADDRESS, 18, "WETH"),
    fee: 500,
  }) as Address;
  const poolContract = getContract({
    address: pool,
    abi: IUniswapV3Pool__factory.abi,
    client: publicClient,
  });
  const npm = getContract({
    address: UNIV3_NPM,
    abi: IUniswapV3NonfungiblePositionManager__factory.abi,
    client: publicClient,
  });

  before(async () => {
    blockNumber = (await publicClient.getBlockNumber()) - 64n;
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
      position: { token0, token1, feeOrTickSpacing: fee },
      slot0: { sqrtPriceX96, tick },
    } = await getPositionDetails(AMM, UNIV3_NPM, 4n, publicClient, blockNumber);
    expect(tokenId).to.be.eq(4n);
    const [_sqrtPriceX96, _tick] = await getContract({
      address: computePoolAddress({
        factoryAddress: UNIV3_FACTORY,
        tokenA: new Token(mainnet.id, token0, 0, "NOT_USED"),
        tokenB: new Token(mainnet.id, token1, 0, "NOT_USED"),
        fee,
      }) as Address,
      abi: IUniswapV3Pool__factory.abi,
      client: publicClient,
    }).read.slot0({ blockNumber });
    expect(sqrtPriceX96).to.be.eq(_sqrtPriceX96);
    expect(tick).to.be.eq(_tick);
  });

  async function verifyPositionDetails(posArr: ContractFunctionReturnType<typeof EphemeralGetPositions__factory.abi>) {
    await Promise.all(
      posArr.map(async ({ tokenId, position, poolFee }) => {
        const [, , token0, token1, fee, tickLower, tickUpper, liquidity] = await npm.read.positions([tokenId], {
          blockNumber,
        });
        expect(position.token0).to.be.eq(token0);
        expect(position.token1).to.be.eq(token1);
        expect(position.feeOrTickSpacing).to.be.eq(fee);
        expect(position.tickLower).to.be.eq(tickLower);
        expect(position.tickUpper).to.be.eq(tickUpper);
        expect(position.liquidity).to.be.eq(liquidity);
        expect(poolFee).to.be.eq(fee);
      }),
    );
  }

  it("Test getting position array", async () => {
    const posArr = await getPositions(
      AMM,
      UNIV3_NPM,
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
    const posArr = await getAllPositionsByOwner(AMM, UNIV3_NPM, owner, publicClient, blockNumber);
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
