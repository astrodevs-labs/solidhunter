use clap::Parser;
use solidhunter_lib::linter::SolidLinter;

use solidhunter_lib::rules::rule_impl::{create_rules_file, parse_rules};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
        create_rules_file(".solidhunter.json");
        println!("Done!");
        return;
    }

    let mut linter : SolidLinter = SolidLinter::new(args.rules_file);
    let mut result = Vec::new();
    for path in args.project_path {
        result.append(&mut linter.parse_folder(path));
    }
    for res in result {
        if res.warnings.len() > 0 {
            for warning in res.warnings {
                println!("{}:{}:{}: warning: {}", warning.range.start.line, warning.range.start.character, warning.range.end.character, warning.message);
            }
        }
    }
}
