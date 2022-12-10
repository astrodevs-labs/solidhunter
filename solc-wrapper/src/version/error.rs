use svm_lib::SolcVmError;
use thiserror::Error;

use semver::Error;

#[derive(Error, Debug)]
pub enum SolcVersionError {
    #[error("SolcVersionError: Something went wrong with sevm")]
    SevmFailed(#[from] SolcVmError),

    #[error("SolcVersionError: Can't do the compuation")]
    ComputationFailed,

    #[error("SolcVersionError: Version failed")]
    VersionFailed(#[from] Error),

    #[error("SolcVersionError: Wrong version of solidity")]
    WrongVersion,
}
