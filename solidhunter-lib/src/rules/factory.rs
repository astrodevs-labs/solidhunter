use std::collections::HashMap;
use crate::rules::types::*;
use crate::types::*;

pub struct RuleFactory {
    _buildables: HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>,
    _rules : Vec<Box<dyn RuleType>>,
}

pub trait FactoryInitializer {
    // function to implement to register rules constructors in the factory
    fn init(&mut self, _buildables: &mut HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>>);
}