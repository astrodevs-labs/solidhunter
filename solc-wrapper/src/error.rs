use svm_lib::SolcVmError;
use thiserror;
use crate::solc::error::CommandError;

#[derive(thiserror::Error, Debug)]
pub enum SolcError {
    #[error("Something went wrong with sevm")]
    SevmFailed,

    #[error("Error from solc")]
    SolcFailed(#[from] CommandError),

    #[error("Can't do the compuation")]
    ComputationFailed,

    #[error("Cannot parse the AST")]
    SerdeJsonError(#[from] serde_json::Error)
}

impl std::convert::From<SolcVmError> for SolcError {
    fn from(err: SolcVmError) -> Self {
        SolcError::SevmFailed
    }
}