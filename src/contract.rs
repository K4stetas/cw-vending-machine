use crate::msg::*;
use crate::error::ContractError;
use cosmwasm_std::{
    Addr, to_json_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};

use crate::state::{MACHINE, MachineItems, OWNER};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;

    let items = MachineItems {
        chocolates: msg.chocolates,
        water_bottles: msg.water_bottles,
        chips: msg.chips,
    };
    MACHINE.save(deps.storage, &items)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        ItemsCount {} => to_json_binary(&query::items_count(deps)?),
    }
}
 
mod query {
    use super::*;

    pub fn items_count(deps: Deps) -> StdResult<Items> {
        let items: MachineItems = MACHINE.load(deps.storage)?;
        let resp = Items {
            chocolates: items.chocolates,
            water_bottles: items.water_bottles,
            chips: items.chips,
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
        GetItem { category } => match category.as_str() {
            "chocolates" => exec::take_chocolates(deps),
            "water bottles" => exec::take_water(deps),
            "chips" => exec::take_chips(deps),
            _ => return Err(ContractError::IncorrectTypeOfItem {})
        },
        Refill { number } => exec::refill(deps, info, number),
    }
}

mod exec {
    use super::*;
    
    pub fn take_chocolates(deps: DepsMut) -> Result<Response, ContractError> {
        let mut items: MachineItems = MACHINE.load(deps.storage)?;
        if 0 == items.chocolates {
            return Err(ContractError::NoSnackLeft {
                category: "chocolates".to_owned(),
            });
        }
        items.chocolates -= 1;

        let event = Event::new("chocolate_is_taken").add_attribute("left", items.chocolates.to_string());

        MACHINE.save(deps.storage, &items)?;
        
        let resp = Response::new().add_event(event);
        
        Ok(resp)
    }

    pub fn take_water(deps: DepsMut) -> Result<Response, ContractError> {
        let mut items: MachineItems = MACHINE.load(deps.storage)?;
        if 0 == items.water_bottles {
            return Err(ContractError::NoSnackLeft {
                category: "water bottles".to_owned(),
            });
        }
        items.water_bottles -= 1;

        let event = Event::new("water_bottle_is_taken").add_attribute("left", items.water_bottles.to_string());

        MACHINE.save(deps.storage, &items)?;
        
        let resp = Response::new().add_event(event);
        
        Ok(resp)
    }

    pub fn take_chips(deps: DepsMut) -> Result<Response, ContractError> {
        let mut items: MachineItems = MACHINE.load(deps.storage)?;
        if 0 == items.chips {
            return Err(ContractError::NoSnackLeft {
                category: "chips".to_owned(),
            });
        }
        items.chips -= 1;

        let event = Event::new("chips_are_taken").add_attribute("left", items.chips.to_string());

        MACHINE.save(deps.storage, &items)?;
        
        let resp = Response::new().add_event(event);
        
        Ok(resp)
    }

    pub fn refill(deps: DepsMut, info: MessageInfo, number: u64) -> Result<Response, ContractError> {
        let owner: Addr = OWNER.load(deps.storage)?;
        if owner != info.sender {
            return Err(ContractError::RefillerIsNotTheOwner {
                sender: info.sender,
            });
        }

        let mut items: MachineItems = MACHINE.load(deps.storage)?;
        
        let chocolates = items.chocolates.checked_add(number);
        let water_bottles = items.water_bottles.checked_add(number);
        let chips = items.chips.checked_add(number);

        match chocolates {
            Some(res) => items.chocolates = res,
            None => return Err(ContractError::TooBigRefill {})
        }

        match water_bottles {
            Some(res) => items.water_bottles = res,
            None => return Err(ContractError::TooBigRefill {})
        }
        
        match chips {
            Some(res) => items.chips = res,
            None => return Err(ContractError::TooBigRefill {})
        }

        let event = Event::new("snacks_are_refilled")
            .add_attribute("chocolates", items.chocolates.to_string())
            .add_attribute("water_bottles", items.water_bottles.to_string())
            .add_attribute("chips", items.chips.to_string());

        MACHINE.save(deps.storage, &items)?;
        
        let resp = Response::new().add_event(event);
        
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn items_count_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { chocolates: 20, water_bottles: 20, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: Items = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::ItemsCount {})
            .unwrap();

        assert_eq!(
            resp,
            Items {
                chocolates: 20,
                water_bottles: 20,
                chips: 20,
            }
        );
    }

    #[test]
    fn take_stored_items() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { chocolates: 20, water_bottles: 0, chips: 20},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let _response = app
            .execute_contract(
                Addr::unchecked("user1"),
                addr.clone(),
                &ExecuteMsg::GetItem { 
                    category: "chocolates".to_owned()
                 },
                &[],
            )
            .unwrap();

        
        let resp: Items = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemsCount {})
            .unwrap();

        assert_eq!(
            resp,
            Items {
                chocolates: 19,
                water_bottles: 0,
                chips: 20,
            }
        );

        let resp = app
            .execute_contract(
                Addr::unchecked("user2"),
                addr.clone(),
                &ExecuteMsg::GetItem { 
                    category: "water bottles".to_owned()
                 },
                &[],
            )
            .unwrap_err();
            
        assert_eq!(
            ContractError::NoSnackLeft {
                category: "water bottles".to_owned(),
            },
            resp.downcast().unwrap()
        );

        let resp: Items = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemsCount {})
            .unwrap();

        assert_eq!(
            resp,
            Items {
                chocolates: 19,
                water_bottles: 0,
                chips: 20,
            }
        );
    }

    #[test]
    fn refill_items() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { chocolates: 20, water_bottles: 0, chips: 20 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let _response = app
            .execute_contract(
                Addr::unchecked("user1"),
                addr.clone(),
                &ExecuteMsg::GetItem { 
                    category: "chocolates".to_owned()
                 },
                &[],
            )
            .unwrap();

        
        let resp: Items = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemsCount {})
            .unwrap();

        assert_eq!(
            resp,
            Items {
                chocolates: 19,
                water_bottles: 0,
                chips: 20,
            }
        );

        let resp = app
            .execute_contract(
                Addr::unchecked("admin1"),
                addr.clone(),
                &ExecuteMsg::Refill { 
                    number: 40
                 },
                &[],
            )
            .unwrap_err();
            
        assert_eq!(
            ContractError::RefillerIsNotTheOwner {
                sender: Addr::unchecked("admin1"),
            },
            resp.downcast().unwrap()
        );

        let resp: Items = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemsCount {})
            .unwrap();

        assert_eq!(
            resp,
            Items {
                chocolates: 19,
                water_bottles: 0,
                chips: 20,
            }
        );

        let _resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr.clone(),
                &ExecuteMsg::Refill { 
                    number: 40
                 },
                &[],
            )
            .unwrap();

        let resp: Items = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemsCount {})
            .unwrap();

        assert_eq!(
            resp,
            Items {
                chocolates: 59,
                water_bottles: 40,
                chips: 60,
            }
        );
    }
}