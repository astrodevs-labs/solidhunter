use clap::builder::Str;
use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{ContractDefinitionChildNodes, decode_location, SourceUnit, SourceUnitChildNodes};

pub struct ContractNamePascalCase {
    data: RuleEntry
}

impl RuleType for ContractNamePascalCase {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {

        let mut res = Vec::new();

        for node in &file.data.nodes {
            match node {
                SourceUnitChildNodes::ContractDefinition(contract) => {
                    if (contract.name.chars().nth(0).unwrap() >= 'a' && contract.name.chars().nth(0).unwrap() <= 'z') ||
                        contract.name.contains("_") ||
                        contract.name.contains("-") {
                        //Untested
                        let location = decode_location(contract.name_location.as_ref().unwrap(), &file.content);
                        res.push(LintDiag {
                            range: Range {
                                start: Position { line: location.0.line as u64, character: location.0.column as u64 },
                                end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                                length: location.0.length as u64,
                            },
                            message: format!("Contract name need to be in pascal case"),
                            severity: Some(self.data.severity),
                            code: None,
                            source: None,
                            uri: file.path.clone(),
                            source_file_content: file.content.clone(),
                        });
                    }
                }
                _ => { continue; }
            }
        }
        res
    }
}

impl ContractNamePascalCase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = ContractNamePascalCase {
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "contract-name-pascalcase".to_string(),
            severity: Severity::WARNING,
            data: vec![]
        }
    }
}