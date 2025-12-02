import { zeroAddress } from "viem";
import { expect } from "chai";
import { ContractFunctionReturnType, createPublicClient, getContract, http } from "viem";
import {
  getPopulatedTicksInRangePCSV4,
  getPositionDetailsPCSV4,
  getPositionsPCSV4,
  PoolKeyStructPCSV4,
} from "../../src/viem";
import {
  EphemeralGetPositionsPCSV4__factory,
  ICLPoolManager__factory,
  ICLPositionManager__factory,
} from "../../typechain";
import { bsc } from "viem/chains";

const PCSV4_POOL_MANAGER = "0xa0FfB9c1CE1Fe56963B0321B32E7A0302114058b";
const PCSV4_POSITION_MANAGER = "0x55f4c8abA71A1e923edC303eb4fEfF14608cC226";
const USDC_ADDRESS = "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d";

describe.only("Pool lens test with UniV4 on mainnet", () => {
  const publicClient = createPublicClient({
    chain: bsc,
    transport: http(`https://bsc-rpc.publicnode.com`),
    batch: {
      multicall: true,
    },
  });
  let blockNumber: bigint;
  const positionManager = getContract({
    address: PCSV4_POSITION_MANAGER,
    abi: ICLPositionManager__factory.abi,
    client: publicClient,
  });
  const poolManager = getContract({
    address: PCSV4_POOL_MANAGER,
    abi: ICLPoolManager__factory.abi,
    client: publicClient,
  });
  const TOKEN_ID: bigint = 156677n;
  const poolKey: PoolKeyStructPCSV4 = {
    currency0: zeroAddress,
    currency1: USDC_ADDRESS,
    hooks: "0x32C59D556B16DB81DFc32525eFb3CB257f7e493d",
    poolManager: PCSV4_POOL_MANAGER,
    fee: 8388608, // dynamic fee tier
    parameters: "0x00000000000000000000000000000000000000000000000000000000000a00c2",
  };
  const poolId = "0xc38e00461a5649e222d41b000698dc8eee20eb8cd7f7efb5bd59115af25f56be" as `0x${string}`;

  before(async () => {
    blockNumber = (await publicClient.getBlockNumber()) - 64n;
    console.log(`Running PCS V4 tests on BSC at block number ${blockNumber}...`);
  });

  it("Test getting populated ticks", async () => {
    const [, tickCurrent, ,] = await poolManager.read.getSlot0([poolId], { blockNumber });
    console.log("Inside hardhat test");
    console.log("tickCurrent: ", tickCurrent);
    const ticks = await getPopulatedTicksInRangePCSV4(PCSV4_POOL_MANAGER, poolKey, tickCurrent, tickCurrent, publicClient, blockNumber);
    await Promise.all(
      ticks.map(async ({ tick, liquidityGross, liquidityNet, feeGrowthOutside0X128, feeGrowthOutside1X128 }) => {
        const { liquidityGross: _liquidityGross, liquidityNet: _liquidityNet, feeGrowthOutside0X128: _feeGrowthOutside0X128, feeGrowthOutside1X128: _feeGrowthOutside1X128 }
          = await poolManager.read.getPoolTickInfo([poolId, tick], { blockNumber });
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
    } = await getPositionDetailsPCSV4(PCSV4_POSITION_MANAGER, TOKEN_ID, publicClient, blockNumber);
    expect(tokenId).to.be.eq(TOKEN_ID);
    const [_sqrtPriceX96, _tick, ,] = await poolManager.read.getSlot0([poolId], { blockNumber });
    expect(sqrtPriceX96).to.be.eq(_sqrtPriceX96);
    expect(tick).to.be.eq(_tick);
  });

  it("Test getting position array", async () => {
    const positions = await getPositionsPCSV4(
      PCSV4_POSITION_MANAGER, [TOKEN_ID, TOKEN_ID],
      publicClient,
      blockNumber,
    );
    await verifyPositionDetails(positions);
  });

  async function verifyPositionDetails(positions: ContractFunctionReturnType<typeof EphemeralGetPositionsPCSV4__factory.abi>) {
    await Promise.all(
      positions.map(async ({ position }) => {
        const liquidity = await positionManager.read.getPositionLiquidity([TOKEN_ID], {
          blockNumber,
        });
        expect(position.liquidity).to.be.eq(liquidity);
      }),
    );
  }
});
