use std::collections::HashMap;;
use crate::rules::rules_base::RuleSeverity::Warn;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub mod code_complexity;
mod function_max_lines;
pub(crate) mod line_maxlen;

// List all rules

use crate::rules::best_practises::code_complexity::CodeComplexity;
use crate::rules::best_practises::line_maxlen::LineMaxLen;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(LineMaxLen::create_default());
    rules.push(CodeComplexity::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn() -> Box<dyn RuleType>> {
    let mut rules = HashMap::new();
    
    rules.push("line-max-len", LineMaxLen::create());
    rules.push("code-complexity", CodeComplexity::create());

    rules
}