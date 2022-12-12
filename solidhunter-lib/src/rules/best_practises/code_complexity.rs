use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;


pub const RULE_ID: &str = "code-complexity";
const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Code complexity is too high";

// Specific
const DEFAULT_COMPLEXITY: u32 = 7;


pub struct CodeComplexity {
    max_complexity: u32,
    data: RuleEntry
}

impl RuleType for CodeComplexity {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        println!("CodeComplexity: trying diagnose file: {}", file.path);
        Vec::new()
    }

    
}
impl CodeComplexity {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = CodeComplexity {
            max_complexity: data.data[0].parse::<u32>().unwrap(),
            data
        };
        Box::new(rule)
    }
    
    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec!["7".to_string()]
        }
    }
}
