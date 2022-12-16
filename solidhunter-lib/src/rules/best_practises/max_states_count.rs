use std::ops::Index;
use crate::linter::SolidFile;
use solc_wrapper::*;
use crate::rules::types::*;
use crate::types::*;


pub struct MaxStatesCount {
    max_states: usize,
    data: RuleEntry
}

impl RuleType for MaxStatesCount {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let mut count = 0;

        // var def => contract def
        for node in file.data.nodes.iter() {
            let contract = match node {
                SourceUnitChildNodes::ContractDefinition(contract) => contract,
                _ => continue
            };
            for node_var in contract.nodes.iter() {
                let var = match node_var {
                    ContractDefinitionChildNodes::VariableDeclaration(var) => var,
                    _ => continue
                };
                count += 1;
                if count > self.max_states {
                    let location = decode_location(&var.src, &file.content);
                    res.push(LintDiag {
                        range: Range {
                            start: Position { line: location.0.line as u64, character: location.0.column as u64},
                            end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                            length: location.0.length as u64
                        },
                        message: format!("Too many states: {}", count),
                        severity: Some(self.data.severity),
                        code: None,
                        source: None,
                        uri: file.path.clone(),
                        source_file_content: file.content.clone()
                    });
                }
            }
        }
        res
    }
}

impl MaxStatesCount {

    pub(crate) const RULE_ID: &'static str = "max-states-count";

    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = MaxStatesCount {
            max_states: data.data[0].parse::<usize>().unwrap(),
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: MaxStatesCount::RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec!["15".to_string()]
        }
    }
}