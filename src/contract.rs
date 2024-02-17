use crate::msg::*;
use crate::error::ContractError;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};

use crate::state::{ADMINS, MACHINE, MachineItems};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;
    let items = MachineItems {
        chocolates: msg.chocolates,
        water: msg.water,
        chips: msg.chips,
    };
    MACHINE.save(deps.storage, &items)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_json_binary(&query::greet()?),
        AdminsList {} => to_json_binary(&query::admins_list(deps)?),
        Items {} => to_json_binary(&query::items_count(deps)?),
    }
}
 
mod query {

    use super::*;

    pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp = AdminsListResp { admins };
        Ok(resp)
    }
    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }

    pub fn items_count(deps: Deps) -> StdResult<ItemsCount> {
        let items: MachineItems = MACHINE.load(deps.storage)?;
        let resp = ItemsCount {
            chocolates: items.get_amount_chocolates(),
            water: items.get_amount_water(),
            chips: items.get_amount_chips(),
        };

        Ok(resp)
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        AddMembers { admins } => exec::add_members(deps, info, admins),
        Leave {} => exec::leave(deps, info).map_err(Into::into),
    }
}

mod exec {
    use super::*;

    pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_admins = ADMINS.load(deps.storage)?;
        if !curr_admins.contains(&info.sender) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let events = admins
            .iter()
            .map(|admin| Event::new("admin_added").add_attribute("addr", admin));
        
        let resp = Response::new()
            .add_events(events)
            .add_attribute("action", "add_members")
            .add_attribute("added_count", admins.len().to_string());

        let admins: StdResult<Vec<_>> = admins
            .into_iter()
            .map(|addr| deps.api.addr_validate(&addr))
            .collect();

        // check wether some admin want to be added additional time
        for admin in admins.as_ref().unwrap() {
            if curr_admins.contains(&admin) {
                return Err(ContractError::AlreadyExistsInTheList {
                    sender: info.sender,
                });
            }
        }
        
        curr_admins.append(&mut admins?);
        ADMINS.save(deps.storage, &curr_admins)?;

        Ok(resp)
    }

    pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
            let admins = admins
                .into_iter()
                .filter(|admin| *admin != info.sender)
                .collect();
            Ok(admins)
        })?;

        Ok(Response::new())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn greet_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![], chocolates: 20, water: 20, chips: 20},
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

    #[test]
    fn instantiation() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![], chocolates: 20, water: 20, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(resp, AdminsListResp { admins: vec![] });

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()], chocolates: 20, water: 20, chips: 20},
                &[],
                "Contract 2",
                None,
            )
            .unwrap();

        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(
            resp,
            AdminsListResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );
    }

    #[test]
    fn unauthorized() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![], chocolates: 20, water: 20, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user")
            },
            err.downcast().unwrap()
        );
    }

    #[test]
    fn add_and_check() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec!["admin1".to_owned(), "admin2".to_owned()], chocolates: 20, water: 20, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();
        
        let addr_clone = addr.clone();

        let _response = app
            .execute_contract(
                Addr::unchecked("admin1"),
                addr,
                &ExecuteMsg::Leave { },
                &[],
            )
            .unwrap();

        
        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(&addr_clone, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(
            resp,
            AdminsListResp {
                admins: vec![Addr::unchecked("admin2")],
            }
        );

        let addr = addr_clone.clone();

        let _response = app
            .execute_contract(
                Addr::unchecked("admin2"),
                addr_clone,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap();
        
        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(
            resp,
            AdminsListResp {
                admins: vec![Addr::unchecked("admin2"), Addr::unchecked("user")],
            }
        );
    }

    #[test]
    fn add_members() { 
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["owner".to_owned()], chocolates: 20, water: 20, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "add_members"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "added_count")
                .unwrap()
                .value,
            "1"
        );

        let admin_added: Vec<_> = resp
            .events
            .iter()
            .filter(|ev| ev.ty == "wasm-admin_added")
            .collect();
        assert_eq!(admin_added.len(), 1);

        assert_eq!(
            admin_added[0]
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );
    }

    #[test]
    fn add_members_double_adding() { 
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["owner".to_owned()], chocolates: 20, water: 20, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr.clone(),
                &ExecuteMsg::AddMembers {
                    admins: vec!["owner".to_owned()],
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::AlreadyExistsInTheList {
                sender: Addr::unchecked("owner")
            },
            resp.downcast().unwrap()
        );

    }
}