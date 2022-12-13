use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

#[macro_use]
pub mod quotes;

// List all rules

use crate::rules::miscellaneous::quotes::Quotes;
use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(Quotes::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert("quotes".to_string(), Quotes::create);

    rules
}