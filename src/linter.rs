use crate::lint_result::LintResult;

pub struct SolidFile {
    pub data : Ast,
    pub path : String,
    pub content : String,
}

pub struct SolidLinter {
    files : Vec<SolidFile>,
}

impl SolidLinter {
    fn parse_file(filepath: String) -> LintResult{
        LintResult::new()
    }
    
    fn parse_content(filepath: String, content : String) -> LintResult{
        LintResult::new()
    }
    
    fn parse_folder(folder: String) -> LintResult{
        LintResult::new()
    }
}