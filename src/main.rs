use clap::Parser;
use colored::Colorize;
use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::offset_from_range;

use solidhunter_lib::rules::rule_impl::{create_rules_file, parse_rules};
use solidhunter_lib::types::Severity;

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

pub fn severity_to_string(severity: Option<Severity>) -> String {
    match severity {
        Some(Severity::ERROR) => format!("error").red(),
        Some(Severity::WARNING) => format!("warning").yellow(),
        Some(Severity::INFO) => format!("info").blue(),
        Some(Severity::HINT) => format!("hint").green(),
        _ => format!("error").red(),
    }
    .to_string()
}

fn print_diag(diag: &solidhunter_lib::types::LintDiag) {
    let mut padding : String = String::new();
    if diag.range.start.line > 99 {
        padding = " ".repeat(0).to_string();
    } else if diag.range.start.line > 9 {
        padding = " ".repeat(1).to_string();
    } else {
        padding = " ".repeat(2).to_string();
    }
    let offset = offset_from_range(diag.source_file_content.as_str(), &diag.range);
    let code_extract = &diag.source_file_content[offset..((offset as usize) + *&diag.range.length as usize)];

    println!("\n{}: {}", severity_to_string(diag.severity), diag.message);
    println!(
        "  --> {}:{}:{}",
        diag.uri,
        diag.range.start.line,
        diag.range.start.character,
    );
    println!(
        "   |");
    //TODO: add code to print
    println!(
        "{}{}|{}", diag.range.start.line,padding, code_extract);
    println!(
        "   |{}{}", " ".repeat(diag.range.start.character as usize), "^".repeat(diag.range.length as usize));
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

    let mut linter: SolidLinter = SolidLinter::new(args.rules_file);
    let mut result = Vec::new();
    for path in args.project_path {
        result.append(&mut linter.parse_folder(path));
    }
    for res in result {
        match res {
            Ok(diags) => {
                for diag in diags {
                    print_diag(&diag);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
