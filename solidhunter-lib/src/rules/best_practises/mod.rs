use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub mod code_complexity;
mod function_max_lines;
pub(crate) mod line_maxlen;

// List all rules

use crate::rules::best_practises::code_complexity::CodeComplexity;
use crate::rules::best_practises::line_maxlen::LineMaxLen;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(LineMaxLen::create_default());
    rules.push(CodeComplexity::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();
    
    rules["line-max-len"] = LineMaxLen::create;
    rules["code-complexity"] = CodeComplexity::create;

    rules
}