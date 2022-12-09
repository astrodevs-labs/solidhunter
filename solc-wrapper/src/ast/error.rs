use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("AstError: cannot parse the json to AST")]
    JsonParseFailed(#[from] serde_json::Error),
}
