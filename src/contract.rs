use anybuf::Anybuf;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:caller-undefined-msg";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        granter: deps.api.addr_validate(&msg.granter)?,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddGrantee { address } => {
            execute::add_grantee(deps, env.contract.address, info.sender, address)
        }
    }
}

pub mod execute {
    use crate::state::GRANTEES;

    use super::*;

    pub fn add_grantee(
        deps: DepsMut,
        contract_addr: Addr,
        granter: Addr,
        str_grantee: String,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.granter != granter {
            return Err(ContractError::Unauthorized {});
        }

        let grantee = deps.api.addr_validate(&str_grantee)?;
        GRANTEES.save(deps.storage, grantee.clone(), &true)?;

        let coin = Anybuf::new()
            .append_string(1, "axpla")
            .append_string(2, "1000000000000000000");

        let allowance = Anybuf::new()
            .append_repeated_message(1, &[coin])
            .append_uint64(2, 0);

        let msg_allowance = Anybuf::new()
            .append_string(1, "/cosmos.feegrant.v1beta1.BasicAllowance")
            .append_message(2, &allowance);
            

        let msg = Anybuf::new()
            .append_string(1, contract_addr)
            .append_string(2, grantee)
            .append_message(3, &msg_allowance);

        Ok(Response::new()
            .add_message(CosmosMsg::Stargate {
                type_url: "/cosmos.feegrant.v1beta1.MsgGrantAllowance".to_string(),
                value: msg.into_vec().into(),
            })
            .add_attribute("action", "add_grantee"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let _state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: 0 })
    }
}
