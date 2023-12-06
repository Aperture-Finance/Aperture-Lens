use crate::{
    bindings::{
        ephemeral_all_positions_by_owner::{AllPositionsReturn, EphemeralAllPositionsByOwner},
        ephemeral_get_populated_ticks_in_range::{
            EphemeralGetPopulatedTicksInRange, GetPopulatedTicksInRangeReturn, PopulatedTick,
        },
        ephemeral_get_position::{EphemeralGetPosition, GetPositionReturn, PositionState},
        ephemeral_get_positions::{EphemeralGetPositions, GetPositionsReturn},
        ephemeral_pool_positions::{EphemeralPoolPositions, PositionKey},
        ephemeral_pool_slots::{EphemeralPoolSlots, GetSlotsReturn, Slot},
        ephemeral_pool_tick_bitmap::EphemeralPoolTickBitmap,
        ephemeral_pool_ticks::EphemeralPoolTicks,
    },
    call_ephemeral_contract,
};
use ethers::{abi::AbiDecode, prelude::*};
use std::sync::Arc;

pub async fn get_populated_ticks_in_range<M: Middleware>(
    pool: Address,
    tick_lower: i32,
    tick_upper: i32,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<PopulatedTick>, ContractError<M>> {
    match call_ephemeral_contract!(
        EphemeralGetPopulatedTicksInRange<M>,
        (pool, tick_lower, tick_upper),
        GetPopulatedTicksInRangeReturn,
        client,
        block_id
    ) {
        Ok(GetPopulatedTicksInRangeReturn { populated_ticks }) => Ok(populated_ticks),
        Err(err) => Err(err),
    }
}

pub async fn get_position_details<M: Middleware>(
    npm: Address,
    token_id: U256,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<PositionState, ContractError<M>> {
    match call_ephemeral_contract!(
        EphemeralGetPosition<M>,
        (npm, token_id),
        GetPositionReturn,
        client,
        block_id
    ) {
        Ok(GetPositionReturn { state }) => Ok(state),
        Err(err) => Err(err),
    }
}

pub async fn get_positions<M: Middleware>(
    npm: Address,
    token_ids: U256,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<PositionState>, ContractError<M>> {
    match call_ephemeral_contract!(
        EphemeralGetPositions<M>,
        (npm, token_ids),
        GetPositionsReturn,
        client,
        block_id
    ) {
        Ok(GetPositionsReturn { positions }) => Ok(positions),
        Err(err) => Err(err),
    }
}

pub async fn get_all_positions_by_owner<M: Middleware>(
    npm: Address,
    owner: Address,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<PositionState>, ContractError<M>> {
    match call_ephemeral_contract!(
        EphemeralAllPositionsByOwner<M>,
        (npm, owner),
        AllPositionsReturn,
        client,
        block_id
    ) {
        Ok(AllPositionsReturn { positions }) => Ok(positions),
        Err(err) => Err(err),
    }
}

/// Call an ephemeral contract and return the decoded storage slots
macro_rules! get_pool_storage {
    ($contract:ty, $constructor_args:expr, $client:expr, $block_id:expr) => {
        match call_ephemeral_contract!($contract, $constructor_args, GetSlotsReturn, $client, $block_id) {
            Ok(GetSlotsReturn { slots }) => Ok(slots),
            Err(err) => Err(err),
        }
    };
}

pub async fn get_static_slots<M: Middleware>(
    pool: Address,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<Slot>, ContractError<M>> {
    get_pool_storage!(EphemeralPoolSlots<M>, (pool,), client, block_id)
}

pub async fn get_ticks_slots<M: Middleware>(
    pool: Address,
    tick_lower: i32,
    tick_upper: i32,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<Slot>, ContractError<M>> {
    get_pool_storage!(EphemeralPoolTicks<M>, (pool, tick_lower, tick_upper), client, block_id)
}
pub async fn get_tick_bitmap_slots<M: Middleware>(
    pool: Address,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<Slot>, ContractError<M>> {
    get_pool_storage!(EphemeralPoolTickBitmap<M>, (pool,), client, block_id)
}
pub async fn get_positions_slots<M: Middleware>(
    pool: Address,
    positions: Vec<PositionKey>,
    client: Arc<M>,
    block_id: Option<BlockId>,
) -> Result<Vec<Slot>, ContractError<M>> {
    get_pool_storage!(EphemeralPoolPositions<M>, (pool, positions), client, block_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use dotenv::dotenv;
    use ethers::contract::Lazy;
    use futures::future::join_all;
    use std::sync::Arc;

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

    async fn verify_slots(slots: Vec<Slot>, client: Arc<Provider<Http>>) {
        let client = client.as_ref();
        let futures = slots[0..4].iter().map(|slot| async move {
            let data = client
                .get_storage_at(POOL_ADDRESS, H256::from_uint(&slot.slot), Some(*BLOCK_NUMBER))
                .await
                .unwrap();
            assert!(slot.data.eq(&U256::from(data.as_bytes())));
        });
        join_all(futures).await;
    }

    #[tokio::test]
    async fn test_get_static_slots() -> Result<()> {
        let client = make_provider().await;
        let slots = get_static_slots(POOL_ADDRESS.parse()?, client.clone(), Some(*BLOCK_NUMBER)).await?;
        verify_slots(slots, client).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_ticks_slots() -> Result<()> {
        let client = make_provider().await;
        let slots = get_ticks_slots(
            POOL_ADDRESS.parse()?,
            -887272,
            887272,
            client.clone(),
            Some(*BLOCK_NUMBER),
        )
        .await?;
        verify_slots(slots, client).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_tick_bitmap_slots() -> Result<()> {
        let client = make_provider().await;
        let slots = get_tick_bitmap_slots(POOL_ADDRESS.parse()?, client.clone(), Some(*BLOCK_NUMBER)).await?;
        verify_slots(slots, client).await;
        Ok(())
    }

    // #[tokio::test]
    // async fn test_get_positions_slots() -> Result<()> {}
}
