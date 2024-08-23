use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub keys_msg: KeysMsg,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Claim { msg: TokenClaimMsg },
    Keys { msg: KeysMsg },
}

#[cw_serde]
pub struct MigrateMsg {
    pub default_gas_limit: Option<u64>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryClaimResponse)]
    Claims { email: String },
}

#[cw_serde]
pub struct QueryClaimResponse {
    pub claims: Vec<ClaimResponse>,
}

#[cw_serde]
pub struct TokenReceiveMsg {
    pub email: String,
    pub memo: Option<String>,
}

#[cw_serde]
pub struct TokenClaimMsg {
    pub jwt: String,
    pub testing: bool,
}

#[cw_serde]
pub struct KeysMsg {
    pub key1: String,
    pub n1: String,
    pub e1: String,
    pub key2: String,
    pub n2: String,
    pub e2: String,
    pub key3: String,
    pub n3: String,
    pub e3: String,
}

#[cw_serde]
pub struct ClaimResponse {
    pub token: String,
    pub sender: String,
    pub memo: String,
    pub amount: Uint128,
}
