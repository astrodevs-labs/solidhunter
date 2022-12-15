use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use solc_wrapper::ast::ast::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Function contains too much lines";

// specific
pub const DEFAULT_MAX_LINES: usize = 20;

pub struct FunctionMaxLines {
    number_max_lines: usize,
    _data: RuleEntry
}

impl RuleType for FunctionMaxLines {

    fn diagnose(&self, _file: &SolidFile, _files: &Vec<SolidFile>) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let functions = get_all_functions_from_ast(&_file.data.nodes);
        for function in functions {
            let _report = check_function_lines(_file, function, self.number_max_lines);
            if let Some(report) = _report {
                res.push(LintDiag {
                    range: report,
                    severity: Some(Severity::WARNING),
                    code: None,
                    source: None,
                    message: DEFAULT_MESSAGE.to_string(),
                    uri: _file.path.clone(),
                    source_file_content: _file.content.clone()
                });
            }
        }
        res
    }
}


// returns a struct containing the line number of the start and end of the function if it is too long
fn check_function_lines(_file: &SolidFile, function: Box<FunctionDefinition>, nb_max_line: usize) ->  Option<Range> {
    let mut res: Option<Range> = None;
    let function_copy_name_location = &function.src;
    let (_start, _) = decode_location(function_copy_name_location.as_str(), _file.content.as_str());
    let index = function_copy_name_location.split(":").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    let mut function_lines:usize = 0;
    let mut left_bracket:usize = 0;
    let mut right_bracket:usize = 0;
    let mut last_bracket_line:usize = 0;

    for (_, c) in _file.content.chars().enumerate().skip(index) {
        if c == '{' {
            left_bracket += 1;
        }
        if c == '}' {
            right_bracket += 1;
        }
        if c == '\n' {
            function_lines += 1;
        }
        if right_bracket > 0 && left_bracket == right_bracket {
            last_bracket_line = _start.line + function_lines;
            break;
        }
    }
    if function_lines > nb_max_line {
        res = Some(Range {
            start: Position {
                line: _start.line as u64,
                character: _start.column as u64,
            },
            end: Position {
                line: last_bracket_line as u64,
                character: 1,
            },
            length: _file.content.lines().nth(_start.line - 1)?.len() as u64
        });
    }
    res
}

fn get_all_functions_from_ast(ast_nodes: &Vec<SourceUnitChildNodes>) -> Vec<Box<FunctionDefinition>> {
    let mut res = Vec::new();

    for node in ast_nodes {
        let contract = match node {
            SourceUnitChildNodes::ContractDefinition(contract) => contract,
            _ => continue,
        };
        for contract_node in &contract.nodes {
            let function = match contract_node {
                ContractDefinitionChildNodes::FunctionDefinition(function) => function,
                _ => continue,
            };
            res.push(function.clone());
        }
    }
    res
}


impl FunctionMaxLines {

    pub(crate) const RULE_ID: &'static str = "function-max-lines";

    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let max_number_lines = match data.data[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => DEFAULT_MAX_LINES
        };

        let rule  = FunctionMaxLines {
            number_max_lines: max_number_lines,
            _data: data
        };
        Box::new(rule)
    }
    
    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: FunctionMaxLines::RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![DEFAULT_MAX_LINES.to_string()]
            
        }
    }
}
