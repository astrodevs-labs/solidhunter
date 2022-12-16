use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub(crate) mod import_on_top;

// List all rules

use crate::rules::order::import_on_top::ImportOnTop;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(ImportOnTop::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "import-on-top".to_string(), ImportOnTop::create);

    rules
}