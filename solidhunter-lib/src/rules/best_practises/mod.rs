use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub mod line_maxlen;
pub mod max_states_count;
pub mod function_max_lines;
pub mod reason_string;

// List all rules

use crate::rules::best_practises::line_maxlen::LineMaxLen;
use crate::rules::best_practises::max_states_count::MaxStatesCount;
use crate::rules::best_practises::reason_string::ReasonString;
use crate::rules::best_practises::function_max_lines::FunctionMaxLines;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(LineMaxLen::create_default());
    rules.push(MaxStatesCount::create_default());
    rules.push(FunctionMaxLines::create_default());
    rules.push(ReasonString::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "line-max-len".to_string(), LineMaxLen::create);
    rules.insert(MaxStatesCount::RULE_ID.to_string(), MaxStatesCount::create);
    rules.insert(FunctionMaxLines::RULE_ID.to_string(), FunctionMaxLines::create);
    rules.insert(reason_string::RULE_ID.to_string(), ReasonString::create);

    rules
}