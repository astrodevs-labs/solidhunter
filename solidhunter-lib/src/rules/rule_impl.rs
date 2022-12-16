use std::collections::HashMap;
use crate::rules::create_default_rules;
use crate::rules::types::*;


// Untested
fn merge_rules(rules: &mut Vec<RuleEntry>, new_rules: &Vec<RuleEntry>) {
    let mut new_rules_map = HashMap::new();
    for rule in new_rules {
        new_rules_map.insert(rule.id.clone(), rule);
    }

    for rule in rules {
        if let Some(new_rule) = new_rules_map.get(&rule.id) {
            rule.severity = new_rule.severity.clone();
            rule.data = new_rule.data.clone();
        }
    }
}

pub fn create_rules_file(path: &str) {
    let mut rules = Rules {
        name: "solidhunter".to_string(),
        includes: vec![],
        plugins: vec![],
        rules: create_default_rules(),
    };
    let serialized = serde_json::to_string_pretty(&rules).unwrap();

    std::fs::write(path, serialized).unwrap();
}

type RulesResult = Result<Rules, RulesError>;

pub fn parse_rules(path: &str) -> RulesResult {
    let mut rules = Rules {
        name: String::new(),
        includes: Vec::new(),
        plugins: Vec::new(),
        rules: Vec::new(),
    };

    if !std::path::Path::new(&path).is_file() {
        return Err(RulesError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "Rules file not found")));
    }
    let file = std::fs::read_to_string(path).unwrap();
    let parsed: Rules = serde_json::from_str(&file).unwrap();

    /*
    // Danger zone
    for include in parsed.includes {
        let include_rules = parse_rules(include.as_str());
        merge_rules(&mut rules.rules, &include_rules.unwrap().rules);
    }

    merge_rules(&mut rules.rules, &parsed.rules);
    // End of danger zone
     */

    Ok(parsed)
}


// create rules
/*
#[macro_export]
macro_rules! create_rule {
    ($rule_name:ident, $rule_id:expr, $default_severity:expr, $custom_data:expr, $message:expr) => {
        pub struct $rule_name {
            id: String,
            message: String,
            severity: RuleSeverity,
            data: Vec<String>,
        }

        impl $rule_name {
            pub fn new(severity: Severity, data: Vec<String>) -> $rule_name {
                $rule_name {
                    id: $rule_id.to_string(),
                    message: $message.to_string(),
                    severity,
                    data,
                }
            }
        }
    };
}*/