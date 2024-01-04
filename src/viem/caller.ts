import { Abi, AbiFunction } from "abitype";
import {
  CallExecutionError,
  ContractFunctionReturnType,
  EncodeDeployDataParameters,
  Hex,
  PublicClient,
  decodeFunctionResult,
  encodeDeployData,
} from "viem";

/**
 * Deploy an ephemeral contract which reverts data in the constructor via `eth_call`.
 * @param deployParams The abi, bytecode, and constructor arguments.
 * @param publicClient Viem public client.
 * @param blockNumber Optional block number to query.
 * @returns The result of the contract function call.
 */
export async function callEphemeralContract<TAbi extends Abi>(
  deployParams: EncodeDeployDataParameters<TAbi>,
  publicClient: PublicClient,
  blockNumber?: bigint,
): Promise<ContractFunctionReturnType<TAbi>> {
  try {
    await publicClient.call({
      // @ts-ignore
      data: encodeDeployData(deployParams),
      blockNumber,
    });
  } catch (error) {
    const baseError = (error as CallExecutionError).walk();
    if ("data" in baseError) {
      const abiFunctions = deployParams.abi.filter((x) => x.type === "function");
      if (abiFunctions.length === 1) {
        // @ts-ignore
        return decodeFunctionResult({
          abi: abiFunctions as [AbiFunction],
          data: baseError.data as Hex,
        });
      } else {
        throw new Error("abi should contain exactly one function");
      }
    } else {
      throw error;
    }
  }
  throw new Error("deployment should revert");
}
