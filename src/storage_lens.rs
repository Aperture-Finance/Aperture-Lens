use crate::bindings::ephemeral_storage_lens::{EphemeralStorageLens, EPHEMERALSTORAGELENS_DEPLOYED_BYTECODE};
use ethers::prelude::*;
use std::sync::Arc;

/// Batch `eth_getStorageAt` RPC calls in a single `eth_call` by overriding the target contract's
/// deployed bytecode with `EphemeralStorageLens`
///
/// # Arguments
///
/// * `address`: The contract address to fetch storage from
/// * `slots`: The storage slots to query
/// * `client`: The ethers client
/// * `block_id`: Optional block id to query
///
/// returns: Result<Vec<[u8; 32], Global>, ContractError<M>>
pub async fn get_storage_at<M: Middleware>(
    address: Address,
    slots: Vec<[u8; 32]>,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<[u8; 32]>, ContractError<M>> {
    let block_id = block_id.unwrap_or(BlockId::from(BlockNumber::Latest));
    // override the deployed bytecode at `address`
    let mut state = spoof::state();
    state
        .account(address)
        .code(EPHEMERALSTORAGELENS_DEPLOYED_BYTECODE.clone());
    let lens = EphemeralStorageLens::new(address, client);
    lens.extsload(slots).call_raw().block(block_id).state(&state).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use dotenv::dotenv;
    use futures::future::join_all;

    const POOL_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
    static BLOCK_NUMBER: Lazy<BlockId> = Lazy::new(|| BlockId::from(17000000));
    static RPC_URL: Lazy<String> = Lazy::new(|| {
        dotenv().ok();
        format!(
            "https://mainnet.infura.io/v3/{}",
            std::env::var("INFURA_API_KEY").unwrap()
        )
    });

    async fn make_provider() -> Arc<Provider<Http>> {
        Arc::new(Provider::<Http>::connect(&*RPC_URL).await)
    }

    #[tokio::test]
    async fn test_get_storage_at() -> Result<()> {
        let client = make_provider().await;
        let slots = get_storage_at(
            POOL_ADDRESS.parse::<Address>()?,
            (0..10).map(|i| H256::from_low_u64_be(i).to_fixed_bytes()).collect(),
            client.clone(),
            Some(*BLOCK_NUMBER),
        )
        .await?;
        let slots_ref = slots.as_slice();
        let address = POOL_ADDRESS.parse::<Address>()?;
        let client = client.as_ref();
        let futures = (0..10).map(|i| async move {
            let slot = client
                .get_storage_at(address, H256::from_low_u64_be(i), Some(*BLOCK_NUMBER))
                .await
                .unwrap();
            assert_eq!(slot.to_fixed_bytes(), slots_ref[i as usize]);
        });
        join_all(futures).await;
        Ok(())
    }
}
