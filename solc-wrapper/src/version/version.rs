use crate::utils;

use svm_lib;
use semver::{Version, VersionReq};
use std::{path::PathBuf};

use super::error::SolcVersionError;

pub struct SolcVersion {
}

impl Default for SolcVersion {
    fn default() -> Self {
        svm_lib::setup_home();

        SolcVersion::new()
    }
}

impl SolcVersion {
    pub fn new() -> Self {
        SolcVersion { }
    }

    pub fn find_matching_version(source: &str) -> Result<Version, SolcVersionError> {
        let version_req = Self::source_version_req(source)?;
        let versions = Self::list_installed_versions()?;
        let version = versions.iter().find(|v| version_req.matches(v));
        if version.is_none() {
            let remote_versions = Self::list_remote_versions()?;
            let version = remote_versions.iter().find(|v| version_req.matches(v));
            return version.cloned().ok_or(SolcVersionError::Other);
        }
        version.cloned().ok_or(SolcVersionError::Other)
    }

    pub fn list_installed_versions() -> Result<Vec<Version>, SolcVersionError> {
        svm_lib::installed_versions().map_err(|e| SolcVersionError::from(e))
    }

    pub fn list_remote_versions() -> Result<Vec<Version>, SolcVersionError> {
        svm_lib::blocking_all_versions().map_err(|e| SolcVersionError::from(e))
    }

    pub fn install_version(version: &Version) -> Result<PathBuf, SolcVersionError> {
        svm_lib::blocking_install(version).map_err(|e| SolcVersionError::from(e))
    }

    pub fn find_version_and_install(version: &Version) -> Result<PathBuf, SolcVersionError> {
        // TODO optimize the code to only have to run it once and outside this function possibly
        let versions = svm_lib::installed_versions()?;
        if versions.is_empty() || !versions.contains(&version) {
            Self::install_version(version)
        } else {
            Ok(svm_lib::version_path(&version.to_string()).join("solc-".to_owned() + version.to_string().as_str()))
        }
    }

    pub fn source_version_req(source: &str) -> Result<VersionReq, SolcVersionError> {
        let version =
            utils::find_version_pragma(source).ok_or(SolcVersionError::ComputationFailed)?;
        Self::version_req(version.as_str())
    }

    /// Returns the corresponding SemVer version requirement for the solidity version
    pub fn version_req(version: &str) -> Result<VersionReq, SolcVersionError> {
        let version = version.replace(' ', ",");

        // Somehow, Solidity semver without an operator is considered to be "exact",
        // but lack of operator automatically marks the operator as Caret, so we need
        // to manually patch it? :shrug:
        let exact = !matches!(&version[0..1], "*" | "^" | "=" | ">" | "<" | "~");
        let mut version = VersionReq::parse(&version).map_err(|e| SolcVersionError::ComputationFailed)?;
        if exact {
            version.comparators[0].op = semver::Op::Exact;
        }

        Ok(version)
    }

}