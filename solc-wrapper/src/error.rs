use svm_lib::SolcVmError;
use thiserror;

#[derive(thiserror::Error, Debug, Clone)]
pub enum SolcError {
  #[error("Something went wrong with sevm")]
  SevmFailed,
  #[error("Error from solc")]
  SolcFailed,
  #[error("Can't do the compuation")]
  ComputationFailed

}

impl std::convert::From<SolcVmError> for SolcError {
    fn from(err: SolcVmError) -> Self {
        SolcError::SevmFailed
    }
}