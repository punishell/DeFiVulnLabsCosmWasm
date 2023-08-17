use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Doublin Assets")]
    DoublingAssets {},

    #[error("Missing Data")]
    MissingData {},

    #[error("Only one coin allowed")]
    OnlyOneCoin {},

    #[error("Funds are not allowed in this transaction")]
    FundsError {},

}

