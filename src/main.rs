mod linter;
mod types;

use solc_wrapper::Solc;

fn main() -> Result<(), anyhow::Error> {
    let path = "wow.sol";
    let ast = Solc::execute_on_file(path).unwrap();
    let res = Solc::parse(ast).unwrap();
    let content = "pragma solidity ^0.4.24;";
    let ast2 = Solc::execute_on_content(content).unwrap();
    Ok(())
}
