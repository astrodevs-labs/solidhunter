use solc_wrapper::Solc;

fn main() -> Result<(), anyhow::Error> {
    let path = "./wow.sol";
    let ast = Solc::execute_on_file(path).unwrap();
    println!("{}", ast);
    Ok(())
}
