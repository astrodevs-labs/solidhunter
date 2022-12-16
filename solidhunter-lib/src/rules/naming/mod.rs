use std::collections::HashMap;
use crate::rules::naming::contract_name_pascalcase::ContractNamePascalCase;
use crate::rules::naming::func_name_camelcase::FuncNameCamelCase;
use crate::rules::naming::use_forbidden_name::UseForbiddenName;
use crate::rules::types::{RuleEntry, RuleType};
use crate::rules::naming::func_param_name_camelcase::FuncParamNameCamelcase;
use crate::rules::RuleBuilder;

#[macro_use]
pub(crate) mod func_param_name_camelcase;
pub(crate) mod contract_name_pascalcase;
pub(crate) mod func_name_camelcase;
pub(crate) mod use_forbidden_name;

// List all rules


pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(FuncParamNameCamelcase::create_default());
    rules.push(ContractNamePascalCase::create_default());
    rules.push(FuncNameCamelCase::create_default());
    rules.push(UseForbiddenName::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "func-param-name-camelcase".to_string(), FuncParamNameCamelcase::create);
    rules.insert( "contract-name-pascalcase".to_string(), ContractNamePascalCase::create);
    rules.insert( "func-name-camelcase".to_string(), FuncNameCamelCase::create);
    rules.insert( UseForbiddenName::RULE_ID.to_string(), UseForbiddenName::create);

    rules
}
