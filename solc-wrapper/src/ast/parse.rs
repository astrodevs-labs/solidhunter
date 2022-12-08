use serde_json::json;
use crate::ast::ast::Ast;

pub fn parse_ast(json: &str) -> Result<Ast, serde_json::Error> {
    serde_json::from_str(json)
}
