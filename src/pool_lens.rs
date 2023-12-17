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
    token_ids: Vec<U256>,
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
    use crate::bindings::{
        i_nonfungible_position_manager::INonfungiblePositionManager,
        i_uniswap_v3_pool::{IUniswapV3Pool, MintFilter},
        shared_types::{PositionFull, Slot0},
    };
    use anyhow::Result;
    use dotenv::dotenv;
    use ethers::{
        abi::{encode, Token},
        utils::{get_create2_address_from_hash, keccak256},
    };
    use futures::future::join_all;
    use std::{ops::Sub, str::FromStr};

    const POOL_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
    const NPM_ADDRESS: &str = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
    static POOL_INIT_CODE_HASH: Lazy<Bytes> =
        Lazy::new(|| Bytes::from_str("0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54").unwrap());
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

    /// Computes the address of a Uniswap V3 pool given the factory address, the two tokens, and the fee.
    ///
    /// # Arguments
    ///
    /// * `factory`: The Uniswap V3 factory address
    /// * `token_0`: The first token address
    /// * `token_1`: The second token address
    /// * `fee`: The fee tier
    /// * `init_code_hash`: The init code hash of the pool
    ///
    /// returns: Address
    fn compute_pool_address(
        factory: Address,
        mut token_0: Address,
        mut token_1: Address,
        fee: u32,
        init_code_hash: &Bytes,
    ) -> Address {
        if token_0 > token_1 {
            (token_0, token_1) = (token_1, token_0);
        }
        get_create2_address_from_hash(
            factory,
            keccak256(encode(&vec![
                Token::Address(token_0),
                Token::Address(token_1),
                Token::Uint(U256::from(fee)),
            ])),
            init_code_hash,
        )
    }

    #[tokio::test]
    async fn test_get_position_details() -> Result<()> {
        let client = make_provider().await;
        let PositionState {
            token_id,
            position: PositionFull {
                token_0, token_1, fee, ..
            },
            slot_0: Slot0 {
                sqrt_price_x96, tick, ..
            },
            ..
        } = get_position_details(NPM_ADDRESS.parse()?, 4.into(), client.clone(), Some(*BLOCK_NUMBER)).await?;
        let pool = IUniswapV3Pool::new(
            compute_pool_address(
                "0x1F98431c8aD98523631AE4a59f267346ea31F984".parse()?,
                token_0,
                token_1,
                fee,
                &POOL_INIT_CODE_HASH,
            ),
            client.clone(),
        );
        let (_sqrt_price_x96, _tick, _, _, _, _, _) = pool.slot_0().block(*BLOCK_NUMBER).call().await?;
        assert_eq!(token_id, 4.into());
        assert_eq!(sqrt_price_x96, _sqrt_price_x96);
        assert_eq!(tick, _tick);
        Ok(())
    }

    async fn verify_position_details(
        positions: Vec<PositionState>,
        npm: INonfungiblePositionManager<Provider<Http>>,
    ) -> Result<()> {
        assert!(!positions.is_empty());
        let client = npm.client();
        let mut multicall = Multicall::new(client.clone(), None).await.unwrap();
        multicall.add_calls(
            false,
            positions
                .iter()
                .map(|PositionState { token_id, .. }| npm.positions(*token_id)),
        );
        let alt_positions: Vec<(
            u128,
            Address,
            Address,
            Address,
            u32,
            i32,
            i32,
            u128,
            U256,
            U256,
            u128,
            u128,
        )> = multicall
            .block(match *BLOCK_NUMBER {
                BlockId::Number(n) => n,
                _ => panic!("block id must be a number"),
            })
            .call_array()
            .await?;
        for (
            i,
            &PositionState {
                position:
                    PositionFull {
                        token_0,
                        token_1,
                        fee,
                        tick_lower,
                        tick_upper,
                        liquidity,
                        ..
                    },
                ..
            },
        ) in positions.iter().enumerate()
        {
            let (_, _, _token_0, _token_1, _fee, _tick_lower, _tick_upper, _liquidity, _, _, _, _) = alt_positions[i];
            assert_eq!(token_0, _token_0);
            assert_eq!(token_1, _token_1);
            assert_eq!(fee, _fee);
            assert_eq!(tick_lower, _tick_lower);
            assert_eq!(tick_upper, _tick_upper);
            assert_eq!(liquidity, _liquidity);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_positions() -> Result<()> {
        let client = make_provider().await;
        let positions = get_positions(
            NPM_ADDRESS.parse()?,
            (1..100).map(|i| i.into()).collect(),
            client.clone(),
            Some(*BLOCK_NUMBER),
        )
        .await?;
        let npm = INonfungiblePositionManager::new(NPM_ADDRESS.parse::<Address>()?, client.clone());
        verify_position_details(positions, npm).await
    }

    #[tokio::test]
    async fn test_get_all_positions_by_owner() -> Result<()> {
        let client = make_provider().await;
        let npm = INonfungiblePositionManager::new(NPM_ADDRESS.parse::<Address>()?, client.clone());
        let total_supply = npm.total_supply().block(*BLOCK_NUMBER).call().await?;
        let owner = npm.owner_of(total_supply.sub(1)).block(*BLOCK_NUMBER).call().await?;
        let positions =
            get_all_positions_by_owner(NPM_ADDRESS.parse()?, owner, client.clone(), Some(*BLOCK_NUMBER)).await?;
        verify_position_details(positions, npm).await
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
