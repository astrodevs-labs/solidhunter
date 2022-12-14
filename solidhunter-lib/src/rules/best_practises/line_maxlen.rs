use std::ops::Index;
use crate::linter::SolidFile;
use solc_wrapper::*;
use crate::rules::types::*;
use crate::types::*;

pub struct LineMaxLen {
    max_len: usize,
    data: RuleEntry
}

impl RuleType for LineMaxLen {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut line_idx = 1;

        for line in file.content.lines() {
            if line.len() > self.max_len {
                res.push(LintDiag {
                    range: Range {
                        start: Position { line: line_idx, character: self.max_len as u64},
                        end: Position { line: line_idx, character: line.len() as u64 },
                        length: (line.len() - self.max_len) as u64
                    },
                    message: format!("Line is too long: {}", line.len()),
                    severity: Some(self.data.severity),
                    code: None,
                    source: None,
                    uri: file.path.clone(),
                    source_file_content: file.content.clone()
                });
            }
            line_idx += 1;
        }
        res
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