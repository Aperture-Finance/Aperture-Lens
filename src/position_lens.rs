use crate::{
    bindings::{
        ephemeral_all_positions_by_owner::{AllPositionsReturn, EphemeralAllPositionsByOwner},
        ephemeral_get_position::{EphemeralGetPosition, GetPositionReturn, PositionState},
        ephemeral_get_positions::{EphemeralGetPositions, GetPositionsReturn},
    },
    call_ephemeral_contract,
};
use ethers::{abi::AbiDecode, prelude::*};
use std::sync::Arc;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{
        i_nonfungible_position_manager::INonfungiblePositionManager,
        i_uniswap_v3_pool::IUniswapV3Pool,
        shared_types::{PositionFull, Slot0},
    };
    use anyhow::Result;
    use ethers::{
        abi::{encode, Token},
        utils::{get_create2_address_from_hash, keccak256},
    };
    use std::{ops::Sub, str::FromStr};

    const FACTORY_ADDRESS: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
    const NPM_ADDRESS: &str = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
    static POOL_INIT_CODE_HASH: Lazy<Bytes> =
        Lazy::new(|| Bytes::from_str("0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54").unwrap());
    static BLOCK_NUMBER: Lazy<BlockId> = Lazy::new(|| BlockId::from(17000000));

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
        let client = Arc::new(MAINNET.provider());
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
            compute_pool_address(FACTORY_ADDRESS.parse()?, token_0, token_1, fee, &POOL_INIT_CODE_HASH),
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
        let client = Arc::new(MAINNET.provider());
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
        let client = Arc::new(MAINNET.provider());
        let npm = INonfungiblePositionManager::new(NPM_ADDRESS.parse::<Address>()?, client.clone());
        let total_supply = npm.total_supply().block(*BLOCK_NUMBER).call().await?;
        let owner = npm.owner_of(total_supply.sub(1)).block(*BLOCK_NUMBER).call().await?;
        let positions =
            get_all_positions_by_owner(NPM_ADDRESS.parse()?, owner, client.clone(), Some(*BLOCK_NUMBER)).await?;
        verify_position_details(positions, npm).await
    }
}
