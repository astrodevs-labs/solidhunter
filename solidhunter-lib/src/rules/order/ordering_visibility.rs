use clap::builder::Str;
use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::{ContractDefinitionChildNodes, ContractKind, decode_location, NodeType, SourceLocation, SourceUnit, SourceUnitChildNodes, StateMutability, Visibility};

pub struct OrderingVisibility {
    data: RuleEntry
}

#[derive(Debug)]
pub struct Eval {
    weight: usize,
    src: SourceLocation
}

fn eval_function(contract_child_node: &Vec<ContractDefinitionChildNodes>) -> Vec<Eval>{

    let mut eval = Vec::new();

    for node in contract_child_node {
        match node {
            ContractDefinitionChildNodes::FunctionDefinition(function) => {

                if function.visibility == Some(Visibility::External) {
                    if function.state_mutability == StateMutability::NonPayable {
                        eval.push(Eval { weight: 1, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::View {
                        eval.push(Eval { weight: 2, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Pure {
                        eval.push(Eval { weight: 3, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Payable {
                        eval.push(Eval { weight: 4, src: function.src.clone() })
                    }
                } else if function.visibility == Some(Visibility::Public) {
                    if function.state_mutability == StateMutability::NonPayable {
                        eval.push(Eval { weight: 5, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::View {
                        eval.push(Eval { weight: 6, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Pure {
                        eval.push(Eval { weight: 7, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Payable {
                        eval.push(Eval { weight: 8, src: function.src.clone() })
                    }
                } else if function.visibility == Some(Visibility::Internal) {
                    if function.state_mutability == StateMutability::NonPayable {
                        eval.push(Eval { weight: 9, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::View {
                        eval.push(Eval { weight: 10, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Pure {
                        eval.push(Eval { weight: 11, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Payable {
                        eval.push(Eval { weight: 12, src: function.src.clone() })
                    }
                } else {
                    if function.state_mutability == StateMutability::NonPayable {
                        eval.push(Eval { weight: 13, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::View {
                        eval.push(Eval { weight: 14, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Pure {
                        eval.push(Eval { weight: 15, src: function.src.clone() })
                    } else if function.state_mutability == StateMutability::Payable {
                        eval.push(Eval { weight: 16, src: function.src.clone() })
                    }
                }
            }
            _ => {}
        }
    }

    eval
}

impl RuleType for OrderingVisibility {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {

        let mut res = Vec::new();

        for node in &file.data.nodes {
            match node {
                SourceUnitChildNodes::ContractDefinition(contract) => {
                    let eval = eval_function(&contract.nodes);

                    println!("{:?}", eval);

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
                                message: format!("Function need to be ordered."),
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

impl OrderingVisibility {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = OrderingVisibility {
            data
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: "ordering-visibility".to_string(),
            severity: Severity::WARNING,
            data: vec![]
        }
    }
}