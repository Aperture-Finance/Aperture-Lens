import { Address, PublicClient } from "viem";
import { callEphemeralContract } from "./caller";
import {
  EphemeralAllPositionsByOwner__factory,
  EphemeralGetPosition__factory,
  EphemeralGetPositions__factory,
} from "../../typechain";
import { ammToSolidityDexEnum, AutomatedMarketMakerEnum } from "./amm";

/**
 * Get the position details in a single call by deploying an ephemeral contract via `eth_call`
 * @param npm Nonfungible position manager address.
 * @param positionId Position id.
 * @param publicClient Viem public client.
 * @param blockNumber Optional block number to query.
 * @returns The position details.
 */
export async function getPositionDetails(
  amm: AutomatedMarketMakerEnum,
  npm: Address,
  positionId: bigint,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralGetPosition__factory.abi,
      bytecode: EphemeralGetPosition__factory.bytecode,
      args: [ammToSolidityDexEnum(amm), npm, positionId],
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
  amm: AutomatedMarketMakerEnum,
  npm: Address,
  positionIds: bigint[],
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralGetPositions__factory.abi,
      bytecode: EphemeralGetPositions__factory.bytecode,
      args: [ammToSolidityDexEnum(amm), npm, positionIds],
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
  amm: AutomatedMarketMakerEnum,
  npm: Address,
  owner: Address,
  publicClient: PublicClient,
  blockNumber?: bigint,
) {
  return await callEphemeralContract(
    {
      abi: EphemeralAllPositionsByOwner__factory.abi,
      bytecode: EphemeralAllPositionsByOwner__factory.bytecode,
      args: [ammToSolidityDexEnum(amm), npm, owner],
    },
    publicClient,
    blockNumber,
  );
}
