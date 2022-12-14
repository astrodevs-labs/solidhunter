use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub mod code_complexity;
mod function_max_lines;
pub mod line_maxlen;
pub mod max_states_count;

// List all rules

use crate::rules::best_practises::code_complexity::CodeComplexity;
use crate::rules::best_practises::line_maxlen::LineMaxLen;
use crate::rules::best_practises::max_states_count::MaxStatesCount;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(LineMaxLen::create_default());
    rules.push(CodeComplexity::create_default());
    rules.push(MaxStatesCount::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "line-max-len".to_string(), LineMaxLen::create);
    rules.insert("code-complexity".to_string(), CodeComplexity::create);
    rules.insert(MaxStatesCount::RULE_ID.to_string(), MaxStatesCount::create);

    rules
}