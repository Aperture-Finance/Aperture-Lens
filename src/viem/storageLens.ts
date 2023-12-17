import {
  Address,
  Hex,
  PublicClient,
  RpcTransactionRequest,
  decodeFunctionResult,
  encodeFunctionData,
  toHex,
} from "viem";
import { EphemeralStorageLens__factory } from "../../typechain";

type StateOverrides = {
  [address: Address]: {
    balance?: Hex;
    nonce?: Hex;
    code?: Hex;
    stateDiff?: {
      [slot: Hex]: Hex;
    };
  };
};

/**
 * Call a contract with the given state overrides.
 * @param tx The transaction request.
 * @param overrides The state overrides.
 * @param publicClient A JSON RPC provider that supports `eth_call` with state overrides.
 * @param blockNumber Optional block number to use for the call.
 */
export async function staticCallWithOverrides(
  tx: RpcTransactionRequest,
  overrides: StateOverrides,
  publicClient: PublicClient,
  blockNumber?: bigint,
): Promise<Hex> {
  return (await publicClient.request({
    method: "eth_call",
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    params: [tx, blockNumber ? toHex(blockNumber) : "latest", overrides],
  })) as Hex;
}

/**
 * Batch `eth_getStorageAt` RPC calls in a single `eth_call` by overriding the target contract's
 * deployed bytecode with `EphemeralStorageLens`
 * @param address The contract address to fetch storage from.
 * @param slots The storage slots to fetch.
 * @param publicClient A JSON RPC provider that supports `eth_call` with state overrides.
 * @param blockNumber Optional block number to query.
 */
export async function getStorageAt(
  address: Address,
  slots: Hex[],
  publicClient: PublicClient,
  blockNumber?: bigint,
): Promise<readonly Hex[]> {
  const overrides = {
    [address]: {
      code: "0x60806040526004361061001e5760003560e01c8063dbd035ff14610023575b600080fd5b610036610031366004610081565b61004c565b60405161004391906100f6565b60405180910390f35b60606020600052816020528160051b6040016040845b81831461007a57803554825260209182019101610062565b5050806000f35b6000806020838503121561009457600080fd5b823567ffffffffffffffff808211156100ac57600080fd5b818501915085601f8301126100c057600080fd5b8135818111156100cf57600080fd5b8660208260051b85010111156100e457600080fd5b60209290920196919550909350505050565b6020808252825182820181905260009190848201906040850190845b8181101561012e57835183529284019291840191600101610112565b5090969550505050505056fea164736f6c6343000816000a" as Hex,
    },
  };
  const data = await staticCallWithOverrides(
    {
      to: address,
      from: "0x0000000000000000000000000000000000000000",
      data: encodeFunctionData({
        abi: EphemeralStorageLens__factory.abi,
        functionName: "extsload",
        args: [slots],
      }),
    },
    overrides,
    publicClient,
    blockNumber,
  );
  return decodeFunctionResult({
    abi: EphemeralStorageLens__factory.abi,
    functionName: "extsload",
    data,
  });
}
