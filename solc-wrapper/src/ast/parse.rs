use crate::ast::ast::Ast;

use super::error::AstError;

pub fn parse_ast(json: &str) -> Result<Ast, AstError> {
    serde_json::from_str(json).map_err(|e| AstError::JsonParseFailed(e))
}
