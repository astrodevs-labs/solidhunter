use clap::builder::Str;
use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{ContractDefinitionChildNodes, SourceUnit, SourceUnitChildNodes};

pub struct ContractNamePascalCase {
    enabled: bool,
    data: RuleEntry
}

pub fn get_line_from_offset(file: &SolidFile, offset: usize) -> Position {
    let mut nb_line = 0;
    let mut tmp = offset;

    for line in file.content.lines() {
        if line.len() < tmp {
            tmp -= line.len();
            nb_line += 1;
            continue;
        }
        return Position {
            line: nb_line,
            character: tmp as u64,
        }
    }
    return Position {
        line: 0,
        character: 0,
    }
}

pub fn get_offset(name_location: String) -> usize {

    let v: Vec<&str> = name_location.split(':').collect();

    v[0].parse().unwrap()
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
                        let offset = get_offset(contract.name_location.clone().unwrap());
                        let pos = get_line_from_offset(file, offset);
                        res.push(LintDiag {
                            range: Range {
                                start: Position { line: pos.line, character: pos.character }, end: Position { line: pos.line, character: pos.character + contract.name.len() as u64 }
                            },
                            message: format!("Contract name need to be in pascal case"),
                            severity: Some(self.data.severity),
                            code: None,
                            source: None,
                            uri: file.path.clone()
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
            enabled: data.data[0].parse::<bool>().unwrap(),
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "contract-name-pascalcase".to_string(),
            severity: Severity::WARNING,
            data: vec![true.to_string()]
        }
    }
}