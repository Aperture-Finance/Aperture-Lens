import { AbiParametersToPrimitiveTypes, ExtractAbiFunction } from "abitype";
import { Address, PublicClient } from "viem";
import {
  EphemeralGetPopulatedTicksInRange__factory,
  EphemeralPoolPositions__factory,
  EphemeralPoolSlots__factory,
  EphemeralPoolTickBitmap__factory,
  EphemeralPoolTicks__factory,
} from "../../typechain";
import { callEphemeralContract } from "./caller";

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

export async function getStaticSlots(pool: Address, publicClient: PublicClient, blockNumber?: bigint) {
  return await callEphemeralContract(
    {
      abi: EphemeralPoolSlots__factory.abi,
      bytecode: EphemeralPoolSlots__factory.bytecode,
      args: [pool],
    },
    publicClient,
    blockNumber,
  );
}

export async function getTicksSlots(
  pool: Address,
  tickLower: number,
  tickUpper: number,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralPoolTicks__factory.abi,
      bytecode: EphemeralPoolTicks__factory.bytecode,
      args: [pool, tickLower, tickUpper],
    },
    publicClient,
    blockNumber,
  );
}

export async function getTickBitmapSlots(pool: Address, publicClient: PublicClient, blockNumber?: bigint) {
  return await callEphemeralContract(
    {
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
  pool: Address,
  keys: PositionKey[],
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralPoolPositions__factory.abi,
      bytecode: EphemeralPoolPositions__factory.bytecode,
      args: [pool, keys],
    },
    publicClient,
    blockNumber,
  );
}
