use clap::builder::Str;
use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{ContractDefinitionChildNodes, decode_location, SourceUnit, SourceUnitChildNodes};

pub struct FuncParamNameCamelcase {
    data: RuleEntry
}

impl RuleType for FuncParamNameCamelcase {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {

        let mut res = Vec::new();

        for node in &file.data.nodes {
            match node {
                SourceUnitChildNodes::ContractDefinition(contract) => {
                    for node in &contract.nodes {
                        match node {
                            ContractDefinitionChildNodes::FunctionDefinition(function) => {
                                for parameter in &function.parameters.parameters {
                                    if !(parameter.name.chars().nth(0).unwrap() >= 'a' && parameter.name.chars().nth(0).unwrap() <= 'z') ||
                                        parameter.name.contains("_") ||
                                        parameter.name.contains("-") {
                                        //Untested
                                        let location = decode_location(parameter.name_location.as_ref().unwrap(), &file.content);
                                        res.push(LintDiag {
                                            range: Range {
                                                start: Position { line: location.0.line as u64, character: location.0.column as u64 },
                                                end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                                                length: location.0.length as u64,
                                            },
                                            message: format!("Parameter name need to be in camel case"),
                                            severity: Some(self.data.severity),
                                            code: None,
                                            source: None,
                                            uri: file.path.clone(),
                                            source_file_content: file.content.clone(),
                                        });
                                    }
                                }
                            }
                            _ => { continue; }
                        }
                    }

                }
                _ => { continue; }
            }
        }
        res
    }
}

impl FuncParamNameCamelcase {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = FuncParamNameCamelcase {
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "func-param-name-camelcase".to_string(),
            severity: Severity::WARNING,
            data: vec![]
        }
    }
}