use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

use crate::rules::order::import_on_top::ImportOnTop;
use crate::rules::order::ordering::Ordering;

#[macro_use]
pub(crate) mod ordering;

#[macro_use]
pub(crate) mod import_on_top;

use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(ImportOnTop::create_default());
    rules.push(Ordering::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "import-on-top".to_string(), ImportOnTop::create);
    rules.insert( "ordering".to_string(), Ordering::create);

    rules
}