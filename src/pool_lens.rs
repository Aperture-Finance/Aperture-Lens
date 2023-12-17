use crate::{
    bindings::{
        ephemeral_get_populated_ticks_in_range::{
            EphemeralGetPopulatedTicksInRange, GetPopulatedTicksInRangeReturn, PopulatedTick,
        },
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
    use crate::bindings::i_uniswap_v3_pool::{IUniswapV3Pool, MintFilter};
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
    async fn test_get_populated_ticks_in_range() -> Result<()> {
        let client = make_provider().await;
        let pool = IUniswapV3Pool::new(POOL_ADDRESS.parse::<Address>()?, client.clone());
        let (_, tick_current, _, _, _, _, _) = pool.slot_0().block(BlockId::from(*BLOCK_NUMBER)).call().await?;
        let ticks = get_populated_ticks_in_range(
            POOL_ADDRESS.parse()?,
            tick_current,
            tick_current,
            client.clone(),
            Some(*BLOCK_NUMBER),
        )
        .await?;
        assert!(!ticks.is_empty());
        let mut multicall = Multicall::new(client.clone(), None).await?;
        multicall.add_calls(false, ticks.iter().map(|&PopulatedTick { tick, .. }| pool.ticks(tick)));
        let alt_ticks: Vec<(u128, i128, U256, U256, i64, U256, u32, bool)> = multicall
            .block(match *BLOCK_NUMBER {
                BlockId::Number(n) => n,
                _ => panic!("block id must be a number"),
            })
            .call_array()
            .await?;
        for (
            i,
            &PopulatedTick {
                liquidity_gross,
                liquidity_net,
                ..
            },
        ) in ticks.iter().enumerate()
        {
            let (_liquidity_gross, _liquidity_net, _, _, _, _, _, _) = alt_ticks[i];
            assert_eq!(liquidity_gross, _liquidity_gross);
            assert_eq!(liquidity_net, _liquidity_net);
        }
        Ok(())
    }

    async fn verify_slots(slots: Vec<Slot>, client: Arc<Provider<Http>>) {
        assert!(!slots.is_empty());
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

    #[tokio::test]
    async fn test_get_positions_slots() -> Result<()> {
        let client = make_provider().await;
        let filter = MintFilter::new::<&Provider<Http>, Provider<Http>>(
            Filter::new().from_block(17000000 - 10000).to_block(17000000),
            &client,
        );
        let logs = filter.query().await?;
        let positions = logs
            .iter()
            .map(
                |&MintFilter {
                     owner,
                     tick_lower,
                     tick_upper,
                     ..
                 }| PositionKey {
                    owner,
                    tick_lower,
                    tick_upper,
                },
            )
            .collect();
        let slots = get_positions_slots(POOL_ADDRESS.parse()?, positions, client.clone(), Some(*BLOCK_NUMBER)).await?;
        verify_slots(slots, client).await;
        Ok(())
    }
}
