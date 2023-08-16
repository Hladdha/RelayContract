use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError{
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("WrongTimeStamp")]
    WrongTimeStamp {},

    #[error("WrongTimeStamp")]
    ZeroAmount {},

    #[error("WrongTimeStamp")]
    HashAlreadyExists {},

    #[error("WrongTimeStamp")]
    AritheMaticOverflow
    
}