use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("StdError: {0}")]
    StdError(#[from] StdError),
    #[error("Could not load pubkey into point in G1")]
    InvalidPubkey {},
    #[error("Signature verification failed")]
    InvalidSignature {},
    #[error("No funds were sent with the expected token: {expected_denom}")]
    NoFundsSent { expected_denom: String },
    #[error("No beacon exists in the database")]
    NoBeacon {},
}
