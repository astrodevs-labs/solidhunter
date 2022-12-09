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

pub fn create_rules() -> HashMap<String, fn() -> Box<dyn RuleType>> {
    let mut rules = HashMap::new();
    rules.append(&mut best_practises::create_rules());
    rules
}