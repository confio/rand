use cosmwasm_std::StdError;
use thiserror::Error;

// thiserror implements Display and ToString if you
// set the `#[error("…")]` attribute for all cases
#[derive(Error, Debug)]
pub enum HandleError {
    #[error("StdError: {0}")]
    StdError(#[from] StdError),
    #[error("Signature verification failed")]
    InvalidSignature {},
}
