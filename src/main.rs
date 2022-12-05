use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Project path
    #[arg(short = 'p', long = "path", default_value = ".", help = "Specify project path")]
    project_path: String,
}

fn main() {
    let args = Args::parse();

    println!();
    println!("SolidHunter: Fast and efficient Solidity linter");
    println!("Author[{}] Version[{}] License[{}]", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_VERSION"), "GNU General Public License v3.0");
    println!();

    println!("Working with project path: {}", args.project_path);
}
