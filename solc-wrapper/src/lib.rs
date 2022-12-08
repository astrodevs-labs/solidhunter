mod solc;
mod ast;

use std::{path::PathBuf};
use semver::{Version};
use solc::command::SolcCommand;
use solc::error::CommandError;
use crate::solc::error::CommandType;


pub struct Solc {
}

impl Default for Solc {
    fn default() -> Self {
        Solc { }
    }
}

impl Solc {
    pub fn new() -> Self {
        Solc { }
    }

    pub fn svm_home() -> Option<PathBuf> {
        home::home_dir().map(|dir| dir.join(".svm"))
    }

    pub fn svm_global_version() -> Option<Version> {
        let version =
            std::fs::read_to_string(Self::svm_home().map(|p| p.join(".global_version"))?).ok()?;
        Version::parse(&version).ok()
    }

    /*
    pub fn find_svm_installed_version(version: impl AsRef<str>) -> Result<Option<Self>> {
        let version = version.as_ref();
        let solc = Self::svm_home()
            .ok_or_else(|| Error::solc("svm home dir not found"))?
            .join(version)
            .join(format!("solc-{version}"));

        if !solc.is_file() {
            return Ok(None)
        }
        Ok(Some(Solc::new(solc)))
    }

    pub fn find_or_install_svm_version(version: impl AsRef<str>) -> Result<Self> {
        let version = version.as_ref();
        if let Some(solc) = Solc::find_svm_installed_version(version)? {
            Ok(solc)
        } else {
            Ok(Solc::blocking_install(&version.parse::<Version>()?)?)
        }
    }

    pub fn find_matching_installation(
        versions: &[Version],
        required_version: &VersionReq,
    ) -> Option<Version> {
        // iterate in reverse to find the last match
        versions.iter().rev().find(|version| required_version.matches(version)).cloned()
    }

    pub fn source_version_req(source: &Source) -> Result<VersionReq> {
        let version =
            utils::find_version_pragma(&source.content).ok_or(SolcError::PragmaNotFound)?;
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
            .arg("--ast-compact-json")
            .arg("--stop-after")
            .arg("parsing")
            .arg(path)
            .execute()
            .map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        let res = String::from_utf8(output.stdout)
            .map_err(|e| CommandError { command_type: CommandType::ParseFile, error: e.to_string() })?;
        Ok(String::from(Self::skip_output_header(&res)))
    }

    pub fn execute_on_content(content : &str) -> Result<String, CommandError> {
        let output = SolcCommand::default()
            .arg("--ast-compact-json")
            .arg("--stop-after")
            .arg("parsing")
            .arg("-")
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