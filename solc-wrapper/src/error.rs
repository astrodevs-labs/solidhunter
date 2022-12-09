use thiserror::Error;
use crate::{solc::error::CommandError, version::error::SolcVersionError, ast::error::AstError};
use anyhow;

#[derive(Error, Debug)]
pub enum SolcError {
    #[error("Something went wrong with sevm")]
    SevmFailed(#[from] SolcVersionError),

    #[error("Error from solc")]
    SolcFailed(#[from] CommandError),

    #[error("Can't do the compuation")]
    ComputationFailed,

    #[error("Error from ast pasing")]
    AstFailed(#[from] AstError),

    #[error("Output is empty")]
    OutputIsEmpty,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
