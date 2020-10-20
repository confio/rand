use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("StdError: {0}")]
    StdError(#[from] StdError),
    #[error("Signature verification failed")]
    InvalidSignature {},
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("StdError: {0}")]
    StdError(#[from] StdError),
    #[error("No beacon exists in the database")]
    NoBeacon {},
}
