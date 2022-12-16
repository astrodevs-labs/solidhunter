use std::collections::HashMap;
use crate::rules::types::{RuleEntry, RuleType};

pub mod types;
pub mod rule_impl;
pub mod factory;

// List all rules
pub mod best_practises;
pub mod miscellaneous;
pub mod naming;
pub mod order;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.append(&mut best_practises::create_default_rules());
    rules.append(&mut miscellaneous::create_default_rules());
    rules.append(&mut naming::create_default_rules());
    rules.append(&mut order::create_default_rules());

    rules
}

type RuleBuilder = fn(RuleEntry) -> Box<dyn RuleType>;

pub fn add_rules(rules : &mut HashMap<String, RuleBuilder>, to_add: HashMap<String, RuleBuilder>) {
    for (key, value) in to_add {
        rules.insert(key, value);
    }
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules = HashMap::new();

    add_rules(&mut rules, best_practises::create_rules());
    add_rules(&mut rules, naming::create_rules());
    add_rules(&mut rules, order::create_rules());
    add_rules(&mut rules, miscellaneous::create_rules());

    rules
}