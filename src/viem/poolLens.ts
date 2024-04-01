import { AbiParametersToPrimitiveTypes, ExtractAbiFunction } from "abitype";
import { Address, PublicClient } from "viem";
import {
  EphemeralGetPopulatedTicksInRange__factory,
  EphemeralPCSV3PoolPositions__factory,
  EphemeralPCSV3PoolSlots__factory,
  EphemeralPCSV3PoolTickBitmap__factory,
  EphemeralPCSV3PoolTicks__factory,
  EphemeralPoolPositions__factory,
  EphemeralPoolSlots__factory,
  EphemeralPoolTickBitmap__factory,
  EphemeralPoolTicks__factory,
} from "../../typechain";
import { callEphemeralContract } from "./caller";
import { AutomatedMarketMakerEnum } from "@aperture_finance/uniswap-v3-automation-sdk";

/**
 * Fetches the liquidity within the tick range for the specified pool by deploying an ephemeral contract via `eth_call`.
 * Each tick consumes about 100k gas, so this method may fail if the number of ticks exceeds 3k assuming the provider
 * gas limit is 300m.
 * @param pool The liquidity pool to fetch the tick to liquidity map for.
 * @param tickLower The lower tick to fetch liquidity for.
 * @param tickUpper The upper tick to fetch liquidity for.
 * @param publicClient Viem public client.
 * @param blockNumber Optional block number to query.
 */
export async function getPopulatedTicksInRange(
  pool: Address,
  tickLower: number,
  tickUpper: number,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralGetPopulatedTicksInRange__factory.abi,
      bytecode: EphemeralGetPopulatedTicksInRange__factory.bytecode,
      args: [pool, tickLower, tickUpper],
    },
    publicClient,
    blockNumber,
  );
}

export async function getStaticSlots(
  amm: AutomatedMarketMakerEnum,
  pool: Address,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    amm === AutomatedMarketMakerEnum.enum.PANCAKESWAP_V3
      ? {
          abi: EphemeralPCSV3PoolSlots__factory.abi,
          bytecode: EphemeralPCSV3PoolSlots__factory.bytecode,
          args: [pool],
        }
      : {
          abi: EphemeralPoolSlots__factory.abi,
          bytecode: EphemeralPoolSlots__factory.bytecode,
          args: [pool],
        },
    publicClient,
    blockNumber,
  );
}

export async function getTicksSlots(
  amm: AutomatedMarketMakerEnum,
  pool: Address,
  tickLower: number,
  tickUpper: number,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    amm === AutomatedMarketMakerEnum.enum.PANCAKESWAP_V3
      ? {
          abi: EphemeralPCSV3PoolTicks__factory.abi,
          bytecode: EphemeralPCSV3PoolTicks__factory.bytecode,
          args: [pool, tickLower, tickUpper],
        }
      : {
          abi: EphemeralPoolTicks__factory.abi,
          bytecode: EphemeralPoolTicks__factory.bytecode,
          args: [pool, tickLower, tickUpper],
        },
    publicClient,
    blockNumber,
  );
}

export async function getTickBitmapSlots(
  amm: AutomatedMarketMakerEnum,
  pool: Address,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    amm === AutomatedMarketMakerEnum.enum.PANCAKESWAP_V3
      ? {
          abi: EphemeralPCSV3PoolTickBitmap__factory.abi,
          bytecode: EphemeralPCSV3PoolTickBitmap__factory.bytecode,
          args: [pool],
        }
      : {
          abi: EphemeralPoolTickBitmap__factory.abi,
          bytecode: EphemeralPoolTickBitmap__factory.bytecode,
          args: [pool],
        },
    publicClient,
    blockNumber,
  );
}

export type PositionKey = AbiParametersToPrimitiveTypes<
  ExtractAbiFunction<typeof EphemeralPoolPositions__factory.abi, "getPositions">["inputs"],
  "inputs"
>[1][0];

export async function getPositionsSlots(
  amm: AutomatedMarketMakerEnum,
  pool: Address,
  keys: PositionKey[],
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    amm === AutomatedMarketMakerEnum.enum.PANCAKESWAP_V3
      ? {
          abi: EphemeralPCSV3PoolPositions__factory.abi,
          bytecode: EphemeralPCSV3PoolPositions__factory.bytecode,
          args: [pool, keys],
        }
      : {
          abi: EphemeralPoolPositions__factory.abi,
          bytecode: EphemeralPoolPositions__factory.bytecode,
          args: [pool, keys],
        },
    publicClient,
    blockNumber,
  );
}
