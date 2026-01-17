use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CounterState, STATE};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = CounterState {
        count: 0,
        admins: vec![info.sender],
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

fn ensure_admin(sender: &Addr, admins: &[Addr]) -> Result<(), ContractError> {
    if admins.contains(sender) {
        Ok(())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps, info),
        ExecuteMsg::Decrement {} => decrement(deps, info),
        ExecuteMsg::AddAdmin { address } => add_admin(deps, info, address),
    }
}

fn increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| {
        ensure_admin(&info.sender, &state.admins)?;
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

fn decrement(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| {
        ensure_admin(&info.sender, &state.admins)?;
        state.count -= 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "decrement"))
}

fn add_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| {
        ensure_admin(&info.sender, &state.admins)?;
        let addr = deps.api.addr_validate(&address)?;
        if !state.admins.contains(&addr) {
            state.admins.push(addr);
        }
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "add_admin"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => {
            let state = STATE.load(deps.storage)?;
            to_binary(&state.count)
        }
        QueryMsg::IsAdmin { address } => {
            let state = STATE.load(deps.storage)?;
            let addr = deps.api.addr_validate(&address)?;
            to_binary(&state.admins.contains(&addr))
        }
    }
}
