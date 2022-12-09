use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;


pub struct LineMaxLen {
    max_len: usize,
    data: RuleEntry
}

impl RuleType for LineMaxLen {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        todo!()
    }

    fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = LineMaxLen {
            max_len: data.data[0].parse::<usize>().unwrap(),
            data
        };
        Box::new(rule)
    }

    fn create_default() -> RuleEntry {
        RuleEntry {
            id: "line-max-len".to_string(),
            severity: Severity::Warning,
            data: vec!["120".to_string()]
        }
    }
} 