use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use crate::error::ContractError;
use cosmwasm_schema::{cw_serde};

#[cw_serde]
pub struct MachineItems {
    pub chocolates: u64,
    pub water: u64,
    pub chips: u64,
}

impl MachineItems {
    pub fn get_amount_chocolates(&self) -> u64 {
        self.chocolates
    }

    pub fn get_amount_water(&self) -> u64 {
        self.water
    }

    pub fn get_amount_chips(&self) -> u64 {
        self.chips
    }

    pub fn take_chocolates(&mut self, amount: u64) -> Result<u64, ContractError>  {
        if amount > self.chocolates {
            return Err(ContractError::AmountIsIncorrect {
                amount_to_take: amount,
                amount_in_machine: self.chocolates,
            });
        }
        
        self.chocolates -= amount;
        Ok(self.chocolates)
        
    }

    pub fn take_water(&mut self, amount: u64) -> Result<u64, ContractError>  {
        if amount > self.water {
            return Err(ContractError::AmountIsIncorrect {
                amount_to_take: amount,
                amount_in_machine: self.water,
            });
        }
        
        self.water -= amount;
        Ok(self.water)
    }

    pub fn take_chips(&mut self, amount: u64) -> Result<u64, ContractError>  {
        if amount > self.chips {
            return Err(ContractError::AmountIsIncorrect {
                amount_to_take: amount,
                amount_in_machine: self.chips,
            });
        }
        
        self.chips -= amount;
        Ok(self.chips)
    }
}


pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const MACHINE: Item<MachineItems> = Item::new("machine_items");
