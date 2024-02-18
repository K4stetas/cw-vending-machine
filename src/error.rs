use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("There are no {category} left")]
    NoSnackLeft { category: String },
    #[error("Plese use existed type of item: chocolates, water bottles, chips")]
    IncorrectTypeOfItem {},
    #[error("The number for refill is too big")]
    TooBigRefill {},
    #[error("The {sender} is nor the owner!")]
    RefillerIsNotTheOwner { sender: Addr },
}