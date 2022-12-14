use crate::types::{Range};
use solc_wrapper::ast::ast::{CodeLocation, offset_from_location};

pub mod linter;
pub mod types;
pub mod rules;

pub fn offset_from_range(content: &str, range: &Range) -> usize {
    let loc = CodeLocation {
        line: range.start.line as usize,
        column: range.start.character as usize,
        length: range.length as usize,
    };
    offset_from_location(content, &loc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
