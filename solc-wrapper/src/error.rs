use thiserror::Error;
use crate::{solc::error::CommandError, version::error::SolcVersionError, ast::error::AstError};
use anyhow;
use crate::solc::parsing_error::ParsingError;

#[derive(Error, Debug)]
pub enum SolcError {
    #[error("SolcError: Something went wrong with sevm")]
    SevmFailed(#[from] SolcVersionError),

    #[error("SolcError: Error from solc")]
    SolcFailed(#[from] CommandError),

    #[error("SolcError: Can't do the compuation")]
    ComputationFailed,

    #[error("SolcError: Error from ast pasing")]
    AstFailed(#[from] AstError),

    #[error("SolcError: Output is empty")]
    OutputIsEmpty,

    #[error("SolcError: compiler returned an error without outputing AST")]
    ParsingFailed(#[from] ParsingError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
