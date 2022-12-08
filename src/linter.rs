use crate::lint_result::LintResult;
use solc_wrapper::Solc;

pub struct SolidFile {
    pub data : Ast,
    pub path : String,
    pub content : String,
}

pub struct SolidLinter {
    files : Vec<SolidFile>,
}

impl SolidLinter {
    
    fn file_exists(&self, path : &str) -> bool {
        for file in &self.files {
            if file.path == path {
                return true;
            }
        }
        false
    }
    
    fn update_file_ast(&mut self, path : &str, ast : Ast) {
        for file in &mut self.files {
            if file.path == path {
                file.data = ast;
            }
        }
    }
    
    fn add_file(&mut self, path : &str, ast : Ast, content : &str) {
        let file = SolidFile {
            data : ast,
            path : path.to_string(),
            content : content.to_string(),
        };
        self.files.push(file);
    }

    fn parse_file(filepath: String) -> LintResult{
        
        let ast = Solc::execute_on_file(filepath.to_str()).unwrap();
        let res = Solc::parse(ast).unwrap();
        
        LintResult::new()
    }
    
    fn parse_content(filepath: String, content : String) -> LintResult{
        LintResult::new()
    }
    
    fn parse_folder(folder: String) -> LintResult{
        LintResult::new()
    }
}