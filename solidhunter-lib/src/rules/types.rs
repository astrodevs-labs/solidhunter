use serde::{Serialize, Deserialize};
use crate::linter::SolidFile;
use crate::types::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct RuleEntry
{
    pub id: String,
    pub severity: Severity,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub name: String,
    pub includes: Vec<String>,
    pub plugins: Vec<String>,
    pub rules: Vec<RuleEntry>
}

#[derive(Debug)]
pub enum RulesError {
    IoError(std::io::Error),
}



pub trait RuleType {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag>;
    
    fn create(data: RuleEntry) -> Box<dyn RuleType>;
    
    fn create_default() -> RuleEntry;
}