use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("Admin {sender} is already in the admin list")]
    AlreadyExistsInTheList { sender: Addr },
    #[error("You want to take {amount_to_take}, but the machine have only {amount_in_machine}")]
    AmountIsIncorrect { amount_to_take: u64, amount_in_machine: u64},
}