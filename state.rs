
rust
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Clone, Debug, PartialEq)]
pub struct CounterState {
    pub count: i32,
    pub admins: Vec<Addr>,
}

pub const STATE: Item<CounterState> = Item::new("state");
