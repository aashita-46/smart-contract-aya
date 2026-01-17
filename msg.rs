use cosmwasm_schema::{cw_serde};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Decrement {},
    AddAdmin { address: String },
}

#[cw_serde]
pub enum QueryMsg {
    GetCount {},
    IsAdmin { address: String },
}
