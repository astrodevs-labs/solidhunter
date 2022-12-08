use crate::create_rule;
use crate::rules::rules_base::{Diagnostic, RuleSeverity};

pub const RULE_ID: &str = "code-complexity";
const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Code complexity is too high";

// Specific
const DEFAULT_COMPLEXITY: u32 = 7;

#[derive(Debug, Clone)]
pub struct CodeComplexity {
    id: String,
    message: String,
    severity: RuleSeverity,
    max_complexity: u32,
}

struct AST; // Fake it until you make it

impl CodeComplexity {
    pub fn new(severity: RuleSeverity, data: Vec<String>) -> CodeComplexity {
        CodeComplexity {
            id: RULE_ID.to_string(),
            message: DEFAULT_MESSAGE.to_string(),
            severity,
            max_complexity: DEFAULT_COMPLEXITY,
        }
    }


    fn detect(&self, &ast: AST) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for function in ast.functions {
            let complexity = function.complexity;
            if complexity > self.max_complexity {
                let diagnostic = Diagnostic {
                    severity: self.severity.clone(),
                    message: self.message.clone(),
                    id: self.id.clone(),
                    location: function.location,
                };
                diagnostics.push(diagnostic);
            }
        }
        diagnostics
    }
}

/*
create_rule!(CodeComplexity, "code-complexity", RuleSeverity::Warn, vec!["7".to_string()], "Code complexity is too high");
*/

/*use crate::rules::rules_base::{RuleEntry};

pub struct CodeComplexity {
    pub id: String,
    pub data: Vec<String>,
}

// Specific constructor
impl CodeComplexity {
    // Mandatory
    pub const RULE_ID: String = "code-complexity".to_string();
    const DEFAULT_SEVERITY: String = "warn".to_string();

    // Specific
    const DEFAULT_COMPLEXITY: i32 = 7;

    // create rule entry
    const RULE: RuleEntry = RuleEntry {
        id: CodeComplexity::RULE_ID.to_string(),
        data: vec![CodeComplexity::DEFAULT_SEVERITY.to_string(),
                   CodeComplexity::DEFAULT_COMPLEXITY.to_string()],
    };

    pub fn new(warn: &str, severity: &i32) -> CodeComplexity {
        CodeComplexity {
            id: CodeComplexity::RULE_ID.to_string(),
            data: vec![warn.to_string(), severity.to_string()],
        }
    }
}*/