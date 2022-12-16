use std::collections::HashMap;
use crate::rules::order::ordering_contract::OrderingContract;
use crate::rules::order::ordering_file::OrderingFile;
use crate::rules::order::ordering_visibility::OrderingVisibility;
use crate::rules::types::{RuleEntry, RuleType};

use crate::rules::order::import_on_top::ImportOnTop;

#[macro_use]
pub(crate) mod ordering_contract;
pub(crate) mod ordering_file;
mod ordering_visibility;

#[macro_use]
pub(crate) mod import_on_top;

use crate::rules::RuleBuilder;

pub fn create_default_rules() -> Vec<RuleEntry> {
    let mut rules = Vec::new();

    rules.push(ImportOnTop::create_default());
    rules.push(OrderingContract::create_default());
    rules.push(OrderingFile::create_default());
    rules.push(OrderingVisibility::create_default());

    rules
}

pub fn create_rules() -> HashMap<String, fn(RuleEntry) -> Box<dyn RuleType>> {
    let mut rules :  HashMap<String, RuleBuilder> = HashMap::new();

    rules.insert( "import-on-top".to_string(), ImportOnTop::create);
    rules.insert( "ordering-contract".to_string(), OrderingContract::create);
    rules.insert( "ordering-file".to_string(), OrderingFile::create);
    rules.insert( "ordering-visibility".to_string(), OrderingFile::create);

    rules
}