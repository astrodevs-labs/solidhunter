use clap::builder::Str;
use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{ContractDefinitionChildNodes, ContractKind, decode_location, NodeType, SourceLocation, SourceUnit, SourceUnitChildNodes};

pub struct OrderingFile {
    data: RuleEntry
}

#[derive(Debug)]
pub struct Eval {
    weight: usize,
    src: SourceLocation
}

fn eval_file(source_unit_childs: &Vec<SourceUnitChildNodes>, res: &mut Vec<LintDiag>, severity: Severity, file: &SolidFile) -> Vec<Eval>{

    let mut eval = Vec::new();

    for node in source_unit_childs {
        match node {
            SourceUnitChildNodes::ContractDefinition(contract) => {
                if contract.contract_kind == ContractKind::Interface {
                    eval.push(Eval { weight: 1, src: contract.src.clone() });
                } else if contract.contract_kind == ContractKind::Library {
                    eval.push(Eval { weight: 2, src: contract.src.clone() });
                } else {
                    eval.push(Eval { weight: 3, src: contract.src.clone() });
                }
            }
            SourceUnitChildNodes::ErrorDefinition(error) => {
                let location = decode_location(error.src.as_str(), &file.content);

                if error.node_type == NodeType::EnumDefinition {
                    res.push(LintDiag {
                        range: Range {
                            start: Position { line: location.0.line as u64, character: location.0.column as u64 },
                            end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                            length: location.0.length as u64,
                        },
                        message: format!("Enum can't be outside a contract."),
                        severity: Some(severity),
                        code: None,
                        source: None,
                        uri: file.path.clone(),
                        source_file_content: file.content.clone(),
                    });
                } else if error.node_type == NodeType::StructDefinition {
                    res.push(LintDiag {
                        range: Range {
                            start: Position { line: location.0.line as u64, character: location.0.column as u64 },
                            end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                            length: location.0.length as u64,
                        },
                        message: format!("Struct can't be outside a contract."),
                        severity: Some(severity),
                        code: None,
                        source: None,
                        uri: file.path.clone(),
                        source_file_content: file.content.clone(),
                    });
                }
                //TODO: Edit this when Error definition fixed
            }
            //SourceUnitChildNodes::EnumDefinition(tmp) => eval.push(Eval { weight: 2, src: tmp.src.clone() }),
            //SourceUnitChildNodes::StructDefinition(tmp) => eval.push(Eval { weight: 3, src: tmp.src.clone() }),
            _ => { continue; }
        }
    }

    eval
}

impl RuleType for OrderingFile {
    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {

        let mut res = Vec::new();
        let eval = eval_file(&file.data.nodes, &mut res, self.data.severity, file);

        if eval.len() > 1 {
            for i in 0..eval.len() - 1 {
                if eval[i].weight > eval[i + 1].weight {
                    let location = decode_location(&eval[i].src, &file.content);
                    res.push(LintDiag {
                        range: Range {
                            start: Position { line: location.0.line as u64, character: location.0.column as u64 },
                            end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                            length: location.0.length as u64,
                        },
                        message: format!("File need to be ordered: Interface -> Library -> Contract"),
                        severity: Some(self.data.severity),
                        code: None,
                        source: None,
                        uri: file.path.clone(),
                        source_file_content: file.content.clone(),
                    });
                }
            }
        }

        res
    }
}

impl OrderingFile {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = OrderingFile {
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "ordering-file".to_string(),
            severity: Severity::WARNING,
            data: vec![]
        }
    }
}