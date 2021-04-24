use cosmwasm_std::{
    log, to_binary, Api, CanonicalAddr, Env, Extern, HandleResponse, HandleResult, HumanAddr,
    InitResponse, InitResult, Querier, QueryResult, StdError, Storage
};
use std::collections::HashSet;
//use cosmwasm_storage::{PrefixedStorage};
//use schemars::_serde_json::value;
use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg, QueryAnswer};
use crate::state::{load, /*may_load, remove,*/ save, UserInfo, State};
use secret_toolkit::utils::{pad_handle_result, pad_query_result, HandleCallback, Query};

//pub const PREFIX_INFOS: &[u8] = b"infos";
pub const BLOCK_SIZE: usize = 256;
pub const CONFIG_KEY: &[u8] = b"config";

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> InitResult {
    let state = State {
        count: msg.count,
        owner: deps.api.canonical_address(&env.message.sender)?,
        users: HashSet::new()
    };

    save(&mut deps.storage, CONFIG_KEY, &state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::Increment {} => try_increment(deps, env),
        HandleMsg::Register { s_key } => try_register(deps, env, s_key),
    }
}

pub fn try_increment<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> HandleResult {
    let mut state: State = load(&deps.storage, CONFIG_KEY)?;
    state.count += 3;
    save(&mut deps.storage, CONFIG_KEY, &state)?;

    Ok(HandleResponse::default())
}

pub fn try_register<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    s_key: String
) -> HandleResult {
    let mut state: State = load(&deps.storage, CONFIG_KEY)?;
    let message_sender = deps.api.canonical_address(&env.message.sender)?;
    state.users.insert(message_sender.as_slice().to_vec());
    save(&mut deps.storage, CONFIG_KEY, &state)?;
    //let mut info_store = PrefixedStorage::new(PREFIX_INFOS, &mut deps.storage);
    let info = UserInfo {
        secret_key: s_key,
        is_valid: true,
    };
    save(&mut deps.storage, message_sender.as_slice(), &info)?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    let response = match msg {
        QueryMsg::GetCount {} => query_count(deps),
        QueryMsg::Search { s_key} => search_result(deps)
    };
    pad_query_result(response, BLOCK_SIZE)
}

fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let state: State = load(&deps.storage, CONFIG_KEY)?;
    to_binary(&QueryAnswer::GetCount {
        count: state.count
    })
    //Ok(CountResponse { count: state.count })
}

fn search_result<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let state: State = load(&deps.storage, CONFIG_KEY)?;
    to_binary(&QueryAnswer::GetCount {
        count: state.count
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }
}
