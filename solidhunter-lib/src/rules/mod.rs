use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

pub mod types;
pub mod rule_impl;
pub mod factory;

// List all rules
pub mod best_practises;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();
    rules.append(&mut best_practises::create_default_rules());
    rules
}

type RuleBuilder = fn(RuleEntry) -> Box<dyn RuleType>;

pub fn add_rules(rules : &mut HashMap<String, RuleBuilder>, toAdd: HashMap<String, RuleBuilder>) {
    for (key, value) in toAdd {
        rules.insert(key, value);
    }
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules = HashMap::new();
    add_rules(&mut rules, best_practises::create_rules());
    rules
}