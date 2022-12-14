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
        println!("Quotes::diagnose");
        let mut res = Vec::new();
        
        for line in file.content.lines() {
            let mut line_idx = 1;
            println!("line: {}", line);
            line.chars().enumerate().for_each(|(idx, c)| {
                if c == '\"' && line.chars().nth(idx - 1).unwrap_or(' ') != '\\' {
                    println!("Found quote at line {} char {}", line_idx, idx);
                    res.push(LintDiag {
                        range: Range {
                            start: Position { line: line_idx, character: 0 as u64},
                            end: Position { line: line_idx, character: (idx) as u64 }
                        },
                        message: format!("Use single quotes instead of double quotes"),
                        severity: Some(self.data.severity),
                        code: None,
                        source: None,
                        uri: file.path.clone()
                    });
                }
                line_idx += 1;
            });
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