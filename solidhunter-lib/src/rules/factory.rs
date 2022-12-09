use std::collections::HashMap;
use crate::rules::types::*;
use crate::rules;
use crate::rules::create_rules;
use crate::types::*;

pub struct RuleFactory {
    _buildables: HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>,
    _rules : Vec<Box<dyn RuleType>>,
}

impl RuleFactory {
    pub fn register_rules(&mut self)
    {
        _buildables.append(create_rules())
    }
    
    pub fn create_rule(&self, rule: RuleEntry) -> Box<dyn RuleType>
    {
        let rule_type = _buildables.get(&rule.id);
        if rule_type.is_none() {
            panic!("Rule {} not found", &rule.id);
        }
        rule_type.unwrap()(rule)
    }
}