use crate::linter::SolidFile;
use solc_wrapper::SourceUnit;
use crate::rules::types::*;
use crate::types::*;


pub struct LineMaxLen {
    max_len: usize,
    data: RuleEntry
}

impl RuleType for LineMaxLen {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        println!("LineMaxLen: trying diagnose file: {}", file.path);
        Vec::new()
    }

    
} 

impl LineMaxLen {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = LineMaxLen {
            max_len: data.data[0].parse::<usize>().unwrap(),
            data
        };
        Box::new(rule)
    }
    
    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "line-max-len".to_string(),
            severity: Severity::WARNING,
            data: vec!["80".to_string()]
        }
    }
}