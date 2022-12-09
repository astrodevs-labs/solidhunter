use crate::types::*;

use glob::glob;

use solc_wrapper::Solc;
use solc_wrapper::Ast;
use solc_wrapper::Node;

use crate::{declare_rule, lint_result};

pub struct SolidFile {
    pub data: Ast,
    pub path: String,
    pub content: String,
}

pub struct SolidLinter {
    files: Vec<SolidFile>,
}

impl SolidLinter {
    
    fn file_exists(&self, path: &str) -> bool {
        for file in &self.files {
            if file.path == path {
                return true;
            }
        }
        false
    }

    fn update_file_ast(&mut self, path: &str, ast: Ast) {
        for file in &mut self.files {
            if file.path == path {
                file.data = ast.clone();
            }
        }
    }

    fn add_file(&mut self, path: &str, ast: Ast, content: &str) {
        let file = SolidFile {
            data: ast,
            path: path.to_string(),
            content: content.to_string(),
        };
        self.files.push(file);
    }

    pub fn parse_file(&mut self, filepath: String) -> LintResult{
        
        let ast = Solc::execute_on_file(filepath.to_str()).unwrap();
        let res = Solc::parse(ast).unwrap();

        if self.file_exists(filepath.to_str()) {
            self.update_file_ast(filepath.to_str(), res);
        } else {
            self.add_file(filepath.to_str(), res, "");
        }
        // TODO: analyze the ast to generate diagnostics
        LintResult::new()
    }
    
    pub fn parse_content(&mut self, filepath: String, content : String) -> LintResult{
        let ast = Solc::execute_on_content(content.as_str()).unwrap();
        let res = Solc::parse(ast).unwrap();

        if self.file_exists(filepath.to_str()) {
            self.update_file_ast(filepath.to_str(), res);
        } else {
            self.add_file(filepath.to_str(), res, content.as_str());
        }
        // TODO: analyze the ast to generate diagnostics
        LintResult::new()
    }
    
    pub fn parse_folder(&mut self, folder: String) -> Vec<LintResult> {
        let mut result: Vec<LintResult> = Vec::new();

        for entry in glob(&*(folder + "/**/*.sol"))? {
            result.push(self.parse_file(entry.unwrap().into_os_string().into_string().unwrap()));
        }
        result
    }
}
