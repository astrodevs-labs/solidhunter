mod rules;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Project path
    #[arg(short = 'p', long = "path", default_value = ".", help = "Specify project path")]
    project_path: Vec<String>,

    #[arg(short = 'e', long = "exclude", help = "Exclude part of the project path")]
    ignore_path: Vec<String>,

    #[arg(short = 'r', long = "rules", default_value = ".solidhunter.json", help = "Specify rules file")]
    rules_file: String,

    #[arg(short = 'v', long = "verbose", default_value = "false", help = "Verbose output")]
    verbose: bool,

    #[arg(short = 'i', long = "init", default_value = "false", help = "Initialize rules file")]
    init: bool,
}

fn init_rules_file(path: &str) {
    let mut rules = rules::create_rules();

    rules.rules.insert("rule-one".to_string(), "value-one".to_string());

    let serialized = serde_json::to_string_pretty(&rules).unwrap();
    std::fs::write(path, serialized).unwrap();
}

fn main() {
    let args = Args::parse();

    println!();
    println!("SolidHunter: Fast and efficient Solidity linter");
    println!("By {} - v{} - GNU GPL v3", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_VERSION"));
    println!();

    if args.verbose {
        println!("Verbose output enabled");
        println!("Project path: {:?}", args.project_path);
        println!("Exclude path: {:?}", args.ignore_path);
        println!("Using rules file: {}", args.rules_file);
        println!("Verbose output: {}", args.verbose);
    }

    if args.init {
        println!("Initializing rules file...");
        init_rules_file(".solidhunter.json");
        println!("Done!");
        return;
    }

    let rules = rules::parse_rules(args.rules_file);
    if rules.is_err() {
        println!("Error: {:?}", rules.unwrap_err());
        return;
    }
    println!("{:?}", rules.unwrap());
}
