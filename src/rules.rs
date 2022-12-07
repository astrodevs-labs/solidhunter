use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub includes: Vec<String>,
    pub plugins: Vec<String>,
    pub rules: HashMap<String, String>
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

fn merge_rules(rules: &mut HashMap<String, String>, new_rules: &HashMap<String, String>) {
    for (key, value) in new_rules {
        rules.insert(key.to_string(), value.to_string());
    }
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
