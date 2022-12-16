use std::ops::Index;
use crate::linter::SolidFile;
use solc_wrapper::*;
use solc_wrapper::ast::utils::{get_all_nodes_by_type, Nodes};
use crate::rules::types::*;
use crate::types::*;

pub struct UseForbiddenName {
    data: RuleEntry
}

impl RuleType for UseForbiddenName {

    fn diagnose(&self, file: &SolidFile, files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let blacklist = vec!['I', 'l', 'O'];

        let nodes = get_all_nodes_by_type(file.data.clone(), NodeType::VariableDeclaration);

        for node in nodes {
            let var = match node {
                Nodes::VariableDeclaration(var) => var,
                _ => continue
            };
            if var.name.len() == 1 && blacklist.contains(&var.name.chars().next().unwrap()) {
                let location = decode_location(&var.src, &file.content);
                res.push(LintDiag {
                    range: Range {
                        start: Position { line: location.0.line as u64, character: location.0.column as u64},
                        end: Position { line: location.1.line as u64, character: location.1.column as u64 },
                        length: location.0.length as u64
                    },
                    message: format!("Forbidden variable name: {}", var.name),
                    severity: Some(self.data.severity),
                    code: None,
                    source: None,
                    uri: file.path.clone(),
                    source_file_content: file.content.clone()
                });
            }
        }
        res
    }

    
} 

impl UseForbiddenName {

    pub const RULE_ID : &'static str = "use-forbidden-name";

    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut rule  = UseForbiddenName {
            data
        };
        Box::new(rule)
    }
    
    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: UseForbiddenName::RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}