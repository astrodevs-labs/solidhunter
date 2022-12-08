use regex::{Match, Regex};
use once_cell::sync::Lazy;

pub static RE_SOL_PRAGMA_VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(r"pragma\s+solidity\s+(?P<version>.+?);").unwrap());

pub fn find_version_pragma(contract: &str) -> Option<Match> {
    RE_SOL_PRAGMA_VERSION.captures(contract)?.name("version")
}