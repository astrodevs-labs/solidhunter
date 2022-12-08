mod solc;
mod ast;
mod utils;

use std::{path::PathBuf};
use solc::command::SolcCommand;
use crate::ast::ast::Ast;
use crate::ast::parse::parse_ast;
use solc::error::CommandError;
use crate::solc::error::CommandType;
use semver::{Version, VersionReq};

use svm_lib;

mod error;
pub use error::SolcError;

pub struct Solc {
}

impl Default for Solc {
    fn default() -> Self {
        svm_lib::setup_home();

        Solc { }
    }
}

impl Solc {
    pub fn new() -> Self {
        Solc { }
    }

    pub fn find_matching_version(source: &str) -> Result<Version, SolcError> {
        let version_req = Self::source_version_req(source)?;
        let versions = Self::list_installed_versions()?;
        let version = versions.iter().find(|v| version_req.matches(v));
        if version.is_none() {
            let remote_versions = Self::list_remote_versions()?;
            let version = remote_versions.iter().find(|v| version_req.matches(v));
            return version.cloned().ok_or(SolcError::ComputationFailed);
        }
        version.cloned().ok_or(SolcError::ComputationFailed)
    }

    pub fn list_installed_versions() -> Result<Vec<Version>, SolcError> {
        svm_lib::installed_versions().map_err(|e| SolcError::from(e))
    }

    pub fn list_remote_versions() -> Result<Vec<Version>, SolcError> {
        svm_lib::blocking_all_versions().map_err(|e| SolcError::from(e))
    }

    pub fn install_version(version: &Version) -> Result<PathBuf, SolcError> {
        svm_lib::blocking_install(version).map_err(|e| SolcError::from(e))
    }

    pub fn find_version_and_install(version: &Version) -> Result<PathBuf, SolcError> {
        // TODO optimize the code to only have to run it once and outside this function possibly
        let versions = svm_lib::installed_versions()?;
        if versions.is_empty() || !versions.contains(&version) {
            svm_lib::blocking_install(version).map_err(|e| SolcError::from(e))
        } else {
            Ok(svm_lib::version_path(&version.to_string()).join("solc-".to_owned() + version.to_string().as_str()))
        }
    }

    pub fn source_version_req(source: &str) -> Result<VersionReq, SolcError> {
        let version =
            utils::find_version_pragma(source).ok_or(SolcError::ComputationFailed)?;
        Self::version_req(version.as_str())
    }

    /// Returns the corresponding SemVer version requirement for the solidity version
    pub fn version_req(version: &str) -> Result<VersionReq, SolcError> {
        let version = version.replace(' ', ",");

        // Somehow, Solidity semver without an operator is considered to be "exact",
        // but lack of operator automatically marks the operator as Caret, so we need
        // to manually patch it? :shrug:
        let exact = !matches!(&version[0..1], "*" | "^" | "=" | ">" | "<" | "~");
        let mut version = VersionReq::parse(&version).map_err(|e| SolcError::ComputationFailed)?;
        if exact {
            version.comparators[0].op = semver::Op::Exact;
        }

        Ok(version)
    }

    fn skip_output_header(output: &str) -> &str {
        let idx = output.find("{").expect("No { found");
        &output[idx..]
    }

    pub fn execute_on_file(path: &str) -> Result<String, CommandError> {
        let content = std::fs::read_to_string(path).map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        let version = Self::find_matching_version( content.as_str()).map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;

        let version_path = Self::find_version_and_install(&version).map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;

        let output = SolcCommand::new(version_path)
            .args(["--ast-compact-json", "--stop-after", "parsing", path])
            .execute()
            .map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        if output.stderr.len() > 0 {
            return Err(CommandError { command_type: CommandType::ParseFile, error: String::from_utf8(output.stderr).unwrap() });
        }
        let res = String::from_utf8(output.stdout)
            .map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        Ok(String::from(Self::skip_output_header(&res)))
    }

    pub fn execute_on_content(content: &str) -> Result<String, CommandError> {
        let version = Self::find_matching_version( content).map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        let version_path = Self::find_version_and_install(&version).map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;

        let output = SolcCommand::new(version_path)
            .args(["--ast-compact-json", "--stop-after", "parsing", "-"])
            .execute_with_input(content)
            .map_err(|e| CommandError { command_type: CommandType::ParseStdin, error: e.to_string() })?;
        if output.stderr.len() > 0 {
            return Err(CommandError { command_type: CommandType::ParseStdin, error: String::from_utf8(output.stderr).unwrap() });
        }
        String::from_utf8(output.stdout)
            .map_err(|e| CommandError { command_type: CommandType::ParseStdin, error: e.to_string() })
    }

    pub fn extract_ast_file(filepath: String) -> Result<Ast, SolcError> {
        let output = Self::execute_on_file(filepath.as_str())?;
        parse_ast(output.as_str()).map_err(|e| SolcError::SerdeJsonError(e))
    }

    pub fn extract_ast_content(content: String) -> Result<Ast, SolcError> {
        let output = Self::execute_on_content(&content)?;
        parse_ast(output.as_str()).map_err(|e| SolcError::SerdeJsonError(e))
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
