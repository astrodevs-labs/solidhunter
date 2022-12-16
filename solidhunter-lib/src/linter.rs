use std::fs;
use crate::types::*;
use crate::rules::types::*;
use crate::rules::factory::RuleFactory;
use crate::rules::rule_impl::{create_rules_file, parse_rules};

use glob::glob;
use solc_wrapper::{Solc, SourceUnit};

pub struct SolidFile {
    pub data: SourceUnit,
    pub path: String,
    pub content: String,
}

pub struct SolidLinter {
    files: Vec<SolidFile>,
    rule_factory: RuleFactory,
    rules : Vec<Box<dyn RuleType>>,
}

impl SolidLinter {
    fn _create_rules(&mut self, rules_config:& String, _first: bool)
    {
        let res = parse_rules(rules_config.as_str());
        match res {
            Ok(rules) => {
                for rule in rules.rules {
                    self.rules.push(self.rule_factory.create_rule(rule));
                }
            }
            Err(_) => {
                create_rules_file(rules_config.as_str());
                if _first {
                    self._create_rules(rules_config, false);
                }
            }
        }
    }
    pub fn initalize(&mut self, rules_config: &String)
    {
        self.rule_factory.register_rules();
        self._create_rules(&rules_config, true);
    }
    pub fn new() -> SolidLinter {
        let mut linter : SolidLinter = SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::new(),
            rules: Vec::new(),
        };
        return linter;
    }

    fn file_exists(&self, path: &str) -> bool {
        for file in &self.files {
            if file.path == path {
                return true;
            }
        }
        false
    }

    fn update_file_ast(&mut self, path: &str, ast: SourceUnit) {
        for file in &mut self.files {
            if file.path == path {
                file.data = ast.clone();
            }
        }
    }

    fn add_file(&mut self, path: &str, ast: SourceUnit, content: &str) {
        let file = SolidFile {
            data: ast,
            path: String::from(path),
            content: String::from(content),
        };
        self.files.push(file);
    }

    pub fn parse_file(&mut self, filepath: String) -> LintResult{
        let res = Solc::default().extract_ast_file(filepath.clone());
        
        if res.is_err() {
            println!("{:?}", res);
            return Err(LintError::SolcError(res.err().unwrap()));
        }
        if self.file_exists(filepath.as_str()) {
            self.update_file_ast(filepath.as_str(), res.expect("ast not found"));
        } else {
            let content = fs::read_to_string(filepath.clone()).map_err(|e| LintError::IoError(e))?;
            self.add_file(filepath.as_str(), res.expect("ast not found"), content.as_str());
        }
        let mut res : Vec<LintDiag> = Vec::new();

        for rule in &self.rules {
            let mut diags = rule.diagnose(&self.files[0], &self.files);
            res.append(&mut diags);
        }
        Ok(res)
    }

    pub fn parse_content(&mut self, filepath: String, content : &String) -> LintResult {
        let res = Solc::default().extract_ast_content(content.to_string());

        if res.is_err() {
            println!("{:?}", res);
            return Err(LintError::SolcError(res.err().unwrap()));
        }

        if self.file_exists(filepath.as_str()) {
            self.update_file_ast(filepath.as_str(), res.expect("ast not found"));
        } else {
            self.add_file(filepath.as_str(), res.expect("ast not found"), content.as_str());
        }

        let mut res : Vec<LintDiag> = Vec::new();

        for rule in &self.rules {
            let mut diags = rule.diagnose(&self.files[0], &self.files);
            res.append(&mut diags);
        }
        Ok(res)
    }

    pub fn parse_folder(&mut self, folder: String) -> Vec<LintResult> {
        let mut result: Vec<LintResult> = Vec::new();

        for entry in glob(&*(folder + "/**/*.sol")) {
            for path in entry {
                result.push(self.parse_file(String::from(path.unwrap().into_os_string().into_string().unwrap())));
            }
        }
        result
    }
    pub fn delete_file(&mut self, path: String) {
        loop {
            let idx = self.files.iter().position(|x| x.path == path);
            match idx {
                Some(idx) => {
                    self.files.remove(idx);
                }
                None => {
                    break;
                }
            }
        }
    }
}
