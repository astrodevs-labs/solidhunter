mod solc;
mod ast;
mod utils;
mod version;

use solc::command::SolcCommand;
use version::version::SolcVersion;
use ast::ast::Ast;
use ast::parse::parse_ast;

mod error;
pub use error::SolcError;

pub struct Solc {
    version: SolcVersion
}

impl Default for Solc {
    fn default() -> Self {
        Solc::new()
    }
}

impl Solc {
    pub fn new() -> Self {
        Solc { version: SolcVersion::default() }
    }

    fn skip_output_header(output: &str) -> &str {
        let idx = output.find("{").expect("No { found");
        &output[idx..]
    }

    pub fn execute_on_file(path: &str) -> Result<String, SolcError> {
        let content = std::fs::read_to_string(path).map_err(|e| SolcError::Other(anyhow::Error::new(e)))?;
        let version = SolcVersion::find_matching_version( content.as_str())?;

        let version_path = SolcVersion::find_version_and_install(&version)?;

        let output = SolcCommand::new(version_path)
            .args(["--ast-compact-json", "--stop-after", "parsing", path])
            .execute()?;
        if output.stderr.len() > 0 {
            return Err(SolcError::OutputIsEmpty);
        }
        let res = String::from_utf8(output.stdout)
            .map_err(|e| SolcError::Other(anyhow::Error::new(e)))?;
        Ok(String::from(Self::skip_output_header(&res)))
    }

    pub fn execute_on_content(content: &str) -> Result<String, SolcError> {
        let version = SolcVersion::find_matching_version( content)?;
        let version_path = SolcVersion::find_version_and_install(&version)?;

        let output = SolcCommand::new(version_path)
            .args(["--ast-compact-json", "--stop-after", "parsing", "-"])
            .execute_with_input(content)?;
        if output.stderr.len() > 0 {
            return Err(SolcError::OutputIsEmpty);
        }
        String::from_utf8(output.stdout)
            .map_err(|e| SolcError::Other(anyhow::Error::new(e)))
    }

    pub fn extract_ast_file(filepath: String) -> Result<Ast, SolcError> {
        let output = Self::execute_on_file(filepath.as_str())?;
        parse_ast(output.as_str()).map_err(|e| SolcError::AstFailed(e))
    }

    pub fn extract_ast_content(content: String) -> Result<Ast, SolcError> {
        let output = Self::execute_on_content(&content)?;
        parse_ast(output.as_str()).map_err(|e| SolcError::AstFailed(e))
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_output_header_already_formatted() {
        let output = r#"{
            "contracts": {},
            "sources": {},
            "errors": []
        }"#;
        assert_eq!(Solc::skip_output_header(output), output);
    }

    #[test]
    fn test_skip_output_header() {
        let output = r#"ok ======= test ====== \n awesome {
            "contracts": {},
            "sources": {},
            "errors": []
        }"#;
        let expected = r#"{
            "contracts": {},
            "sources": {},
            "errors": []
        }"#;
        assert_eq!(Solc::skip_output_header(output), expected);
    }
}
