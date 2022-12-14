use std::collections::HashMap;
use crate::rules::naming::contract_name_pascalcase::ContractNamePascalCase;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub(crate) mod func_param_name_camelcase;
pub(crate) mod contract_name_pascalcase;

// List all rules

use crate::rules::naming::func_param_name_camelcase::FuncParamNameCamelcase;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(FuncParamNameCamelcase::create_default());
    rules.push(ContractNamePascalCase::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "func-param-name-camelcase".to_string(), FuncParamNameCamelcase::create);
    rules.insert( "contract-name-pascalcase".to_string(), ContractNamePascalCase::create);

    rules
}