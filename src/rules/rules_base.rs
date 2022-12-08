use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub includes: Vec<String>,
    pub plugins: Vec<String>,
    pub rules: HashMap<String, Vec<String>>
}

#[derive(Debug)]
pub enum RulesError {
    IoError(std::io::Error),
}

type RulesResult = Result<Rules, RulesError>;

pub fn create_rules() -> Rules {
    Rules {
        includes: Vec::new(),
        plugins: Vec::new(),
        rules: HashMap::new()
    }
}

pub fn get_predefined_rules() -> HashMap<String, Rules> {
    let mut rules_map = HashMap::new();

    rules_map.insert("best-practises".to_string(), create_rules());
    rules_map.insert("miscellaneous".to_string(), create_rules());
    rules_map.insert("naming".to_string(), create_rules());
    rules_map.insert("order".to_string(), create_rules());
    rules_map.insert("security".to_string(), create_rules());

    let best_practises = &mut rules_map.get_mut("best-practises").unwrap().rules;


    best_practises.insert("code-complexity".to_string(), vec!["warn".to_string(), "7".to_string()]);
    best_practises.insert("constructor-syntax".to_string(), vec!["warn".to_string()]);
    best_practises.insert("function-max-lines".to_string(), vec!["warn".to_string(), "50".to_string()]);
    best_practises.insert("max-line-length".to_string(), vec!["error".to_string(), "120".to_string()]);
    best_practises.insert("max-states-count".to_string(), vec!["warn".to_string(), "15".to_string()]);
    best_practises.insert("no-empty-blocks".to_string(), vec!["warn".to_string()]);
    best_practises.insert("no-unused-vars".to_string(), vec!["warn".to_string()]);
    best_practises.insert("payable-fallback".to_string(), vec!["^_".to_string()]);

    rules_map
}

// Untested
fn merge_rules(rules: &mut HashMap<String, Vec<String>>, new_rules: &HashMap<String, Vec<String>>) {
    for (key, value) in new_rules {
        rules.insert(key.to_string(), value.to_vec());
    }
}

pub fn create_rules_file(path: &str) {
    let mut rules = create_rules();

    rules.rules.insert("rule-one".to_string(),
                       vec!["value-one".to_string(), "value-two".to_string()]);

    let serialized = serde_json::to_string_pretty(&rules).unwrap();
    std::fs::write(path, serialized).unwrap();
}

pub fn parse_rules(path: String) -> RulesResult {
    let mut rules = Rules {
        includes: Vec::new(),
        plugins: Vec::new(),
        rules: HashMap::new()
    };

    if !std::path::Path::new(&path).is_file() {
        return Err(RulesError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "Rules file not found")));
    }
    let file = std::fs::read_to_string(path).unwrap();
    let parsed: Rules = serde_json::from_str(&file).unwrap();

    for include in parsed.includes {
        let include_rules = parse_rules(include);
        merge_rules(&mut rules.rules, &include_rules.unwrap().rules);
    }

    merge_rules(&mut rules.rules, &parsed.rules);

    Ok(rules)
}
