///`Slot(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Slot {
    pub slot: ::ethers::core::types::U256,
    pub data: ::ethers::core::types::U256,
}
///`PositionFull(uint96,address,address,address,uint24,int24,int24,uint128,uint256,uint256,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PositionFull {
    pub nonce: u128,
    pub operator: ::ethers::core::types::Address,
    pub token_0: ::ethers::core::types::Address,
    pub token_1: ::ethers::core::types::Address,
    pub fee: u32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
    pub fee_growth_inside_0_last_x128: ::ethers::core::types::U256,
    pub fee_growth_inside_1_last_x128: ::ethers::core::types::U256,
    pub tokens_owed_0: u128,
    pub tokens_owed_1: u128,
}
///`PositionState(uint256,address,(uint96,address,address,address,uint24,int24,int24,uint128,uint256,uint256,uint128,uint128),(uint160,int24,uint16,uint16,uint16,uint8,bool),uint128,uint8,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PositionState {
    pub token_id: ::ethers::core::types::U256,
    pub owner: ::ethers::core::types::Address,
    pub position: PositionFull,
    pub slot_0: Slot0,
    pub active_liquidity: u128,
    pub decimals_0: u8,
    pub decimals_1: u8,
}
///`Slot0(uint160,int24,uint16,uint16,uint16,uint8,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Slot0 {
    pub sqrt_price_x96: ::ethers::core::types::U256,
    pub tick: i32,
    pub observation_index: u16,
    pub observation_cardinality: u16,
    pub observation_cardinality_next: u16,
    pub fee_protocol: u8,
    pub unlocked: bool,
}
