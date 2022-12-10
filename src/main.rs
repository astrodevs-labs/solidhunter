use solc_wrapper::Solc;

fn main() -> Result<(), anyhow::Error> {
    let path = "wow.sol";
    let ast = Solc::default().extract_ast_file(path.to_string())?;
    println!("{:?}", ast);
    Ok(())
}
