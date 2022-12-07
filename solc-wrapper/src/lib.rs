use std::{process::Command, path::PathBuf, error::Error};
use semver::{Version, VersionReq};

pub struct Solc {
    args: Vec<String>
}

impl Default for Solc {
    fn default() -> Self {
        Solc { args: Vec::new() }
    }
}

impl Solc {
    pub fn new() -> Self {
        Solc { args: Vec::new() }
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

}