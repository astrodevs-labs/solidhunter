use crate::utils;

use semver::{Version, VersionReq};
use std::{path::PathBuf};

use super::error::SolcVersionError;

pub struct SolcVersion {
    global_version_path: PathBuf
}

impl Default for SolcVersion {
    fn default() -> Self {
        SolcVersion::new(Self::get_global_version_path())
    }
}

impl SolcVersion {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        SolcVersion { global_version_path: path.into() }
    }

    pub fn find_matching_version(&self, source: &str) -> Result<Version, SolcVersionError> {
        let version_req = Self::source_version_req(source)?;

        if self.global_version_path.is_file() {
            let installed_versions = Self::list_installed_versions()?;
            let version = installed_versions.iter().find(|v| version_req.matches(v));
            if !version.is_none() {
                return version.cloned().ok_or(SolcVersionError::ComputationFailed);
            }
        }
        let remote_versions = Self::list_remote_versions()?;
        let version = remote_versions.iter().find(|v| version_req.matches(v));
        version.cloned().ok_or(SolcVersionError::ComputationFailed)
    }

    pub fn list_installed_versions() -> Result<Vec<Version>, SolcVersionError> {
        Ok(svm_lib::installed_versions()?)
    }

    pub fn get_global_version_path() -> PathBuf {
        svm_lib::global_version_path()
    }

    pub fn list_remote_versions() -> Result<Vec<Version>, SolcVersionError> {
        Ok(svm_lib::blocking_all_versions()?)
    }

    pub fn install_version(version: &Version) -> Result<PathBuf, SolcVersionError> {
        Ok(svm_lib::blocking_install(version)?)
    }

    pub fn find_version_and_install(&self, version: &Version) -> Result<PathBuf, SolcVersionError> {
        // TODO optimize the code to only have to run it once and outside this function possibly
        if self.global_version_path.is_file() {
            let versions = svm_lib::installed_versions()?;
            if !versions.is_empty() && versions.contains(&version) {
                return Ok(svm_lib::version_path(&version.to_string()).join("solc-".to_owned() + version.to_string().as_str()));
            }
        }
        Self::install_version(version)
    }

    pub fn source_version_req(source: &str) -> Result<VersionReq, SolcVersionError> {
        let version =
            utils::find_version_pragma(source).ok_or(SolcVersionError::WrongVersion)?;
        Self::version_req(version.as_str())
    }

    /// Returns the corresponding SemVer version requirement for the solidity version
    pub fn version_req(version: &str) -> Result<VersionReq, SolcVersionError> {
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

}