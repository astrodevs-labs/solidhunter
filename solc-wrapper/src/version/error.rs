use svm_lib::SolcVmError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolcVersionError {
    #[error("SolcVersionError: Something went wrong with sevm")]
    SevmFailed(#[from] SolcVmError),

    #[error("SolcVersionError: Can't do the compuation")]
    ComputationFailed,

    #[error("SolcVersionError: Cannot parse the version")]
    ParseVersionFailed,

    #[error("SolcVersionError: Something went wrong")]
    Other
}
