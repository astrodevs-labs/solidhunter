use std::{process::Command, io};
use std::io::Write;
use std::process::{Output, Stdio};

const SOLC_EXE: &str = "solc";

pub struct SolcCommand {
    args: Vec<String>
}

impl Default for SolcCommand {
    fn default() -> Self {
        SolcCommand { args: Vec::new() }
    }
}

impl SolcCommand {

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn execute(&self) -> Result<Output, io::Error> {
        Command::new(SOLC_EXE)
            .args(&self.args)
            .stdout(Stdio::piped())
            .output()

    }

    pub fn execute_with_input(&self, input: &str) -> Result<Output, io::Error> {
        let mut cmd = Command::new(SOLC_EXE)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let child_stdin = cmd.stdin.as_mut().unwrap();
            child_stdin.write_all(input.as_bytes())?;
        }
        cmd.wait_with_output()
    }
}