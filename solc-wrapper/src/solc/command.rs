use std::{process::Command, io};
use std::io::Write;
use std::process::{Output, Stdio};

pub struct SolcCommand {
    args: Vec<String>,
    bin_path : String
}

impl Default for SolcCommand {
    fn default() -> Self {
        SolcCommand {
            args: Vec::new(),
            bin_path: "".to_string()
        }
    }
}

impl SolcCommand {

    pub fn new(bin_path: &str) -> Self {
        SolcCommand {
            args: Vec::new(),
            bin_path: bin_path.to_string()
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

    pub fn execute(&self) -> Result<Output, io::Error> {
        Command::new(&self.bin_path)
            .args(&self.args)
            .stdout(Stdio::piped())
            .output()

    }

    pub fn execute_with_input(&self, input: &str) -> Result<Output, io::Error> {
        let mut cmd = Command::new(&self.bin_path)
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