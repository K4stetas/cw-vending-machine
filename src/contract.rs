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
        chocolate_bars: msg.chocolate_bars,
        water_bottles: msg.water_bottles,
        chips_packets: msg.chips_packets,
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
            chocolate_bars: items.chocolate_bars,
            water_bottles: items.water_bottles,
            chips_packets: items.chips_packets,
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
            "chocolate bar" => exec::take_chocolate(deps),
            "water bottle" => exec::take_water(deps),
            "chips packet" => exec::take_chips(deps),
            _ => return Err(ContractError::IncorrectTypeOfItem {})
        },
        Refill { number } => exec::refill(deps, info, number),
    }
}

mod exec {
    use super::*;
    
    pub fn take_chocolate(deps: DepsMut) -> Result<Response, ContractError> {
        let mut items: MachineItems = MACHINE.load(deps.storage)?;
        if 0 == items.chocolate_bars {
            return Err(ContractError::NoSnackLeft {
                category: "chocolate bars".to_owned(),
            });
        }
        items.chocolate_bars -= 1;

        let event = Event::new("chocolate_bar_is_taken").add_attribute("left", items.chocolate_bars.to_string());

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
        if 0 == items.chips_packets {
            return Err(ContractError::NoSnackLeft {
                category: "chips packets".to_owned(),
            });
        }
        items.chips_packets -= 1;

        let event = Event::new("chips_packet_are_taken").add_attribute("left", items.chips_packets.to_string());

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
        
        let chocolate_bars = items.chocolate_bars.checked_add(number);
        let water_bottles = items.water_bottles.checked_add(number);
        let chips_packets = items.chips_packets.checked_add(number);

        match chocolate_bars {
            Some(res) => items.chocolate_bars = res,
            None => return Err(ContractError::TooBigRefill {})
        }

        match water_bottles {
            Some(res) => items.water_bottles = res,
            None => return Err(ContractError::TooBigRefill {})
        }
        
        match chips_packets {
            Some(res) => items.chips_packets = res,
            None => return Err(ContractError::TooBigRefill {})
        }

        let event = Event::new("snacks_are_refilled")
            .add_attribute("chocolate_bars", items.chocolate_bars.to_string())
            .add_attribute("water_bottles", items.water_bottles.to_string())
            .add_attribute("chips_packets", items.chips_packets.to_string());

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
                &InstantiateMsg {
                    chocolate_bars: 20,
                    water_bottles: 20,
                    chips_packets: 20
                },
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
                chocolate_bars: 20,
                water_bottles: 20,
                chips_packets: 20,
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
                &InstantiateMsg {
                    chocolate_bars: 20,
                    water_bottles: 0,
                    chips_packets: 20
                },
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
                    category: "chocolate bar".to_owned()
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
                chocolate_bars: 19,
                water_bottles: 0,
                chips_packets: 20,
            }
        );

        let resp = app
            .execute_contract(
                Addr::unchecked("user2"),
                addr.clone(),
                &ExecuteMsg::GetItem { 
                    category: "water bottle".to_owned()
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
                chocolate_bars: 19,
                water_bottles: 0,
                chips_packets: 20,
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
                &InstantiateMsg {
                    chocolate_bars: 20,
                    water_bottles: 0,
                    chips_packets: 20
                },
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
                    category: "chocolate bar".to_owned()
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
                chocolate_bars: 19,
                water_bottles: 0,
                chips_packets: 20,
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
                chocolate_bars: 19,
                water_bottles: 0,
                chips_packets: 20,
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
                chocolate_bars: 59,
                water_bottles: 40,
                chips_packets: 60,
            }
        );
    }
}