pub struct LintResult {
    pub errors : Vec<ResultElem>,
    pub warnings : Vec<ResultElem>,
    pub infos : Vec<ResultElem>,
    pub hints : Vec<ResultElem>,
}

pub struct ResultElem {

    /// The range at which the message applies.
    pub range: Range,

    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,



    /// The diagnostic's code. Can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<NumberOrString>,


    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    pub uri: Uri,
}


////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////

pub struct Position {
    pub line: u64,
    pub character: u64,
}
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

pub struct Range {
    pub start: Position,
    pub end: Position,
}
pub struct Location {
    pub uri: Uri,
    pub range: Range,
}

pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
