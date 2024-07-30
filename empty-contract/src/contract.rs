use crate::msg::{GreetResp, InstantiateMsg, QueryMsg};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = _msg.admins
    .into_iter()
    .map(|addr| deps.api.addr_validate(&addr))
    .collect();
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_json_binary(&query::greet()?),
    }
}

mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }
}


#[allow(dead_code)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty
) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};
    use crate::msg::{GreetResp, QueryMsg};
    use cosmwasm_std::{
        to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    };
    
    pub fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: Empty,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }
    
    pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        use QueryMsg::*;
    
        match msg {
            Greet {} => to_json_binary(&query::greet()?),
        }
    }
    
    mod query {
        use super::*;
    
        pub fn greet() -> StdResult<GreetResp> {
            let resp = GreetResp {
                message: "Hello World".to_owned(),
            };
    
            Ok(resp)
        }
    }
    #[test]
    fn greet_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {admins: vec![]},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}
    


