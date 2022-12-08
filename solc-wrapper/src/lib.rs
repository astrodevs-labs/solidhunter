mod solc;
mod utils;

use std::{path::PathBuf};
use solc::command::SolcCommand;
use solc::error::CommandError;
use crate::solc::error::CommandType;
use semver::{Version};

use svm_lib;

mod error;
pub use error::SolcError;

pub struct Solc {
    pub solc: PathBuf,
    pub args: Vec<String>
}

impl Default for Solc {
    fn default() -> Self {
        svm_lib::setup_home();

        Solc { solc: PathBuf::from("solc"), args: Vec::new() }
    }
}

impl Solc {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Solc { solc: path.into(), args: Vec::new() }
    }

    pub fn install_version(&self, version: &Version) -> Result<PathBuf, SolcError> {
        svm_lib::blocking_install(version).map_err(|e| SolcError::from(e))
    }

    pub fn find_version_and_install(&self, version: &Version) -> Result<PathBuf, SolcError> {
        // TODO optimize the code to only have to run it once and outside this function possibly
        let versions = svm_lib::installed_versions()?;
        if versions.is_empty() || !versions.contains(&version) {
            svm_lib::blocking_install(version).map_err(|e| SolcError::from(e))
        } else {
            Ok(svm_lib::version_path(&version.to_string()))
        }
    }

    /*
    pub fn source_version_req(source: &str) -> Result<VersionReq> {
        let version =
            utils::find_version_pragma(source).ok_or(SolcError::PragmaNotFound)?;
        Self::version_req(version.as_str())
    }

    /// Returns the corresponding SemVer version requirement for the solidity version
    pub fn version_req(version: &str) -> Result<VersionReq> {
        let version = version.replace(' ', ",");

        // Somehow, Solidity semver without an operator is considered to be "exact",
        // but lack of operator automatically marks the operator as Caret, so we need
        // to manually patch it? :shrug:
        let exact = !matches!(&version[0..1], "*" | "^" | "=" | ">" | "<" | "~");
        let mut version = VersionReq::parse(&version)?;
        if exact {
            version.comparators[0].op = semver::Op::Exact;
        }

        Ok(version)
    }
    */

    fn skip_output_header(output: &str) -> &str {
        let idx = output.find("{").expect("No { found");
        &output[idx..]
    }

    pub fn execute_on_file(path : &str) -> Result<String, CommandError> {
        let output = SolcCommand::default()
            .args(["--ast-compact-json", "--stop-after", "parsing", path])
            .execute()
            .map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        let res = String::from_utf8(output.stdout)
            .map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        Ok(String::from(Self::skip_output_header(&res)))
    }

    pub fn execute_on_content(content : &str) -> Result<String, CommandError> {
        let output = SolcCommand::default()
            .args(["--ast-compact-json", "--stop-after", "parsing", "-"])
            .execute_with_input(content)
            .map_err(|e| CommandError { command_type: CommandType::ParseStdin, error: e.to_string() })?;
        if !output.stderr.len() > 0 {
            return Err(CommandError { command_type: CommandType::ParseStdin, error: String::from_utf8(output.stderr).unwrap() });
        }
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        String::from_utf8(output.stdout)
            .map_err(|e| CommandError { command_type: CommandType::ParseStdin, error: e.to_string() })
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

    /*
    #[test]
    fn test_execute_on_file() {
        let path = "wow.sol";
        let ast = Solc::execute_on_file(path).unwrap();
        println!("{}", ast);
    }

    #[test]
    fn test_execute_on_content() {
        let content = "pragma solidity ^0.5.0;
     */
}
