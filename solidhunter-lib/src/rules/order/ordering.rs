use clap::builder::Str;
use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{ContractDefinitionChildNodes, ContractKind, decode_location, NodeType, SourceLocation, SourceUnit, SourceUnitChildNodes};

pub struct Ordering {
    data: RuleEntry
}

#[derive(Debug)]
pub struct Eval {
    weight: usize,
    src: SourceLocation
}

fn eval_file(source_unit_childs: &Vec<SourceUnitChildNodes>) -> Vec<Eval>{

    let mut eval = Vec::new();

    for node in source_unit_childs {
        match node {
            SourceUnitChildNodes::ErrorDefinition(error) => {
                if error.node_type == NodeType::EnumDefinition {
                    eval.push(Eval { weight: 1, src: error.src.clone() });
                } else if error.node_type == NodeType::StructDefinition {
                    eval.push(Eval { weight: 2, src: error.src.clone() });
                }
                //TODO: Remove this when Error definition fixed
            }
            SourceUnitChildNodes::EnumDefinition(tmp) => eval.push(Eval { weight: 2, src: tmp.src.clone() }),
            SourceUnitChildNodes::StructDefinition(tmp) => eval.push(Eval { weight: 3, src: tmp.src.clone() }),
            SourceUnitChildNodes::ContractDefinition(contract) => {
                if contract.contract_kind == ContractKind::Interface {
                    eval.push(Eval { weight: 3, src: contract.src.clone() });
                } else if contract.contract_kind == ContractKind::Library {
                    eval.push(Eval { weight: 4, src: contract.src.clone() });
                } else {
                    eval.push(Eval { weight: 5, src: contract.src.clone() });
                }
            }
            _ => { continue; }
        }
    }

    eval
}

fn eval_contract(contract_child_node: &Vec<ContractDefinitionChildNodes>) -> Vec<Eval>{

    let mut eval = Vec::new();

    for node in contract_child_node {
        match node {
            ContractDefinitionChildNodes::ErrorDefinition(tmp) => {
                //TODO: Remove this when Error definition fixed
                if tmp.node_type == NodeType::EnumDefinition {
                    eval.push(Eval { weight: 3, src: tmp.src.clone() });
                } else if tmp.node_type == NodeType::StructDefinition {
                    eval.push(Eval { weight: 2, src: tmp.src.clone() });
                }
            },
            ContractDefinitionChildNodes::UsingForDirective(tmp) => eval.push(Eval { weight: 1, src: tmp.src.clone() }),
            ContractDefinitionChildNodes::StructDefinition(tmp) => eval.push(Eval { weight: 2, src: tmp.src.clone() }),
            ContractDefinitionChildNodes::EnumDefinition(tmp) => eval.push(Eval { weight: 3, src: tmp.src.clone() }),
            ContractDefinitionChildNodes::VariableDeclaration(tmp) => eval.push(Eval { weight: 4, src: tmp.src.clone() }),
            ContractDefinitionChildNodes::EventDefinition(tmp) => eval.push(Eval { weight: 5, src: tmp.src.clone() }),
            ContractDefinitionChildNodes::ModifierDefinition(tmp) => eval.push(Eval { weight: 6, src: tmp.src.clone() }),
            ContractDefinitionChildNodes::FunctionDefinition(tmp) => eval.push(Eval { weight: 7, src: tmp.src.clone() }),
            _ => { continue; }
        }
    }

    eval
}

impl RuleType for Ordering {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {

        let mut res = Vec::new();
        let eval = eval_file(&file.data.nodes);

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
                        message: format!("File need to be ordered: Using for -> Struct -> Enum -> Variable -> Event -> Modifier -> Function"),
                        severity: Some(self.data.severity),
                        code: None,
                        source: None,
                        uri: file.path.clone(),
                        source_file_content: file.content.clone(),
                    });
                }
            }
        }

        for node in &file.data.nodes {
            match node {
                SourceUnitChildNodes::ContractDefinition(contract) => {
                    let eval = eval_contract(&contract.nodes);

                    if eval.len() < 2 { continue; }
                    for i in 0..eval.len() - 1 {
                        if eval[i].weight > eval[i + 1].weight {
                            let location = decode_location(&eval[i].src, &file.content);
                            res.push(LintDiag {
                                range: Range {
                                    start: Position { line: location.0.line as u64, character: location.0.column as u64 },
                                    end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                                    length: location.0.length as u64,
                                },
                                message: format!("Contract need to be ordered: Enum -> Struct -> Interface -> Library -> Contract"),
                                severity: Some(self.data.severity),
                                code: None,
                                source: None,
                                uri: file.path.clone(),
                                source_file_content: file.content.clone(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        res
    }
}

impl Ordering {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = Ordering {
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "ordering".to_string(),
            severity: Severity::WARNING,
            data: vec![]
        }
    }
}