use crate::ast::ast::Ast;

use super::error::AstError;

pub fn parse_ast(json: &str) -> Result<Ast, AstError> {
    Ok(serde_json::from_str(json)?)
}
