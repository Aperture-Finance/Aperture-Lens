import { AbiParametersToPrimitiveTypes, ExtractAbiFunction } from "abitype";
import { Address, PublicClient } from "viem";
import {
  EphemeralAllPositionsByOwner__factory,
  EphemeralGetPopulatedTicksInRange__factory,
  EphemeralGetPosition__factory,
  EphemeralGetPositions__factory,
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

/**
 * Get the position details in a single call by deploying an ephemeral contract via `eth_call`
 * @param npm Nonfungible position manager address.
 * @param positionId Position id.
 * @param publicClient Viem public client.
 * @param blockNumber Optional block number to query.
 * @returns The position details.
 */
export async function getPositionDetails(
  npm: Address,
  positionId: bigint,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralGetPosition__factory.abi,
      bytecode: EphemeralGetPosition__factory.bytecode,
      args: [npm, positionId],
    },
    publicClient,
    blockNumber,
  );
}

/**
 * Get the state and pool for all positions in a single call by deploying an ephemeral contract via `eth_call`.
 * @param npm Nonfungible position manager address.
 * @param positionIds Position ids.
 * @param publicClient Viem public client.
 * @param blockNumber Optional block number to query.
 * @returns The position details for all positions.
 */
export async function getPositions(
  npm: Address,
  positionIds: bigint[],
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralGetPositions__factory.abi,
      bytecode: EphemeralGetPositions__factory.bytecode,
      args: [npm, positionIds],
    },
    publicClient,
    blockNumber,
  );
}

/**
 * Get the state and pool for all positions of the specified owner by deploying an ephemeral contract via `eth_call`.
 * Each position consumes about 200k gas, so this method may fail if the number of positions exceeds 1500 assuming the
 * provider gas limit is 300m.
 * @param npm Nonfungible position manager address.
 * @param owner The owner of the positions.
 * @param publicClient Viem public client.
 * @param blockNumber Optional block number to query.
 * @returns The position details for all positions of the specified owner.
 */
export async function getAllPositionsByOwner(
  npm: Address,
  owner: Address,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralAllPositionsByOwner__factory.abi,
      bytecode: EphemeralAllPositionsByOwner__factory.bytecode,
      args: [npm, owner],
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
