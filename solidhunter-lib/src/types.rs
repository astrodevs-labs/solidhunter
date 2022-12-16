use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LintError {
    #[error("LintError: Solc error occured")]
    SolcError(#[from] solc_wrapper::SolcError),
    #[error("LintError: Something went wrong with the file")]
    IoError(#[from] std::io::Error),
    #[error("LintError: Something went wrong")]
    LinterError(String),
}


pub type LintResult = Result<Vec<LintDiag>, LintError>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LintDiag {

    /// The range at which the message applies.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    pub severity: Option<Severity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's code. Can be omitted.
    pub code: Option<NumberOrString>,


    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    pub uri: Uri,

    #[serde(rename="sourceFileContent")]
    pub source_file_content: String
}


////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub line: u64,
    pub character: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Severity {
    /// Reports an error.
    ERROR = 1,
    /// Reports a warning.
    WARNING = 2,
    /// Reports an information.
    INFO = 3,
    /// Reports a hint.
    HINT = 4
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
    pub length: u64,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
