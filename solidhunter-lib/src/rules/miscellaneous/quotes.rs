use std::ops::Index;
use crate::linter::SolidFile;
use solc_wrapper::*;
use crate::rules::types::*;
use crate::types::*;


pub struct Quotes {
    data: RuleEntry
}

impl RuleType for Quotes {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut line_idx = 1;

        for line in file.content.lines() {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c == '\'' && line.chars().nth(idx - 1).unwrap_or(' ') != '\\' {
                    res.push(LintDiag {
                        range: Range {
                            start: Position { line: line_idx, character: idx as u64},
                            end: Position { line: line_idx, character: idx as u64 },
                            length: 1 as u64,
                        },
                        message: format!("Use double quotes instead of single quote"),
                        severity: Some(self.data.severity),
                        code: None,
                        source: None,
                        uri: file.path.clone(),
                        source_file_content: file.content.clone()
                    });
                }
            });
            line_idx += 1;
        }
        res
    }

} 

impl Quotes {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = Quotes {
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "quotes".to_string(),
            severity: Severity::ERROR,
            data: vec![]
        }
    }
}