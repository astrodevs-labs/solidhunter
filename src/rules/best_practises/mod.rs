use crate::rules::best_practises::code_complexity::CodeComplexity;
use crate::rules::rules_base::RuleSeverity::Warn;

#[macro_use]
pub mod code_complexity;
mod function_max_lines;

// List all rules
pub fn list_rules() -> Vec<String> {
    let mut rules = Vec::new();
    rules
}