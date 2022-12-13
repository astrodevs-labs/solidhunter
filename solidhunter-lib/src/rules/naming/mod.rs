use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub(crate) mod func_param_name_camelcase;

// List all rules

use crate::rules::naming::func_param_name_camelcase::FuncParamNameCamelcase;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(FuncParamNameCamelcase::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "func-param-name-camelcase".to_string(), FuncParamNameCamelcase::create);

    rules
}