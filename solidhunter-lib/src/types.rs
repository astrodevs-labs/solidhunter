use serde::{Serialize, Deserialize};

pub struct LintResult {
    pub errors : Vec<LintDiag>,
    pub warnings : Vec<LintDiag>,
    pub infos : Vec<LintDiag>,
    pub hints : Vec<LintDiag>,
}

#[derive(Clone)]
pub struct LintDiag {

    /// The range at which the message applies.
    pub range: Range,

    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    pub severity: Option<Severity>,



    /// The diagnostic's code. Can be omitted.
    pub code: Option<NumberOrString>,


    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    pub uri: Uri,
}


////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Position {
    pub line: u64,
    pub character: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Severity(i32);


impl Severity {
    /// Reports an error.
    pub const ERROR: Severity = Severity(1);
    /// Reports a warning.
    pub const WARNING: Severity = Severity(2);
    /// Reports an information.
    pub const INFO: Severity = Severity(3);
    /// Reports a hint.
    pub const HINT: Severity = Severity(4);
}

#[derive(Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}
pub struct Location {
    pub uri: Uri,
    pub range: Range,
}

#[derive(Clone)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
