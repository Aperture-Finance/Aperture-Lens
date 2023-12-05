import { Abi, AbiFunction } from "abitype";
import {
  CallExecutionError,
  ContractFunctionResult,
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
): Promise<ContractFunctionResult<TAbi>> {
  try {
    await publicClient.call({
      data: encodeDeployData(deployParams),
      blockNumber,
    });
  } catch (error) {
    const baseError = (error as CallExecutionError).walk();
    if ("data" in baseError) {
      const abiFunctions = deployParams.abi.filter((x) => x.type === "function");
      if (abiFunctions.length === 1) {
        return decodeFunctionResult({
          abi: abiFunctions as [AbiFunction],
          data: baseError.data as Hex,
        }) as ContractFunctionResult<TAbi>;
      } else {
        throw new Error("abi should contain exactly one function");
      }
    } else {
      throw error;
    }
  }
  throw new Error("deployment should revert");
}
