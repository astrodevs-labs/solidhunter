use crate::ast::ast::SourceUnit;

use super::error::AstError;

pub fn parse_ast(json: &str) -> Result<SourceUnit, AstError> {
    Ok(serde_json::from_str(json)?)
}
