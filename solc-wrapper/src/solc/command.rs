use std::{process::Command};
use std::io::Write;
use std::process::{Output, Stdio};
use std::{path::PathBuf};

use super::error::{CommandError, CommandType};

pub struct SolcCommand {
    args: Vec<String>,
    bin_path : PathBuf
}

impl Default for SolcCommand {
    fn default() -> Self {
        SolcCommand::new("solc")
    }
}

impl SolcCommand {

    pub fn new(path: impl Into<PathBuf>) -> Self {
        SolcCommand {
            args: Vec::new(),
            bin_path: path.into()
        }
    }

    pub fn arg<T: Into<String>>(mut self, arg: T) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self = self.arg(arg);
        }
        self
    }

    pub fn execute(&self) -> Result<Output, CommandError> {
        Command::new(&self.bin_path)
            .args(&self.args)
            .stdout(Stdio::piped())
            .output()
            .map_err(|e| CommandError { error: e.to_string(), command_type: CommandType::ParseFile })
    }

    pub fn execute_with_input(&self, input: &str) -> Result<Output, CommandError> {
        let mut cmd = Command::new(&self.bin_path)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| CommandError { error: e.to_string(), command_type: CommandType::ParseStdin })?;

        {
            let child_stdin = cmd.stdin.as_mut().unwrap();
            child_stdin.write_all(input.as_bytes()).map_err(|e| CommandError { error: e.to_string(), command_type: CommandType::ParseStdin })?;
        }
        cmd.wait_with_output().map_err(|e| CommandError { error: e.to_string(), command_type: CommandType::ParseStdin })
    }
}