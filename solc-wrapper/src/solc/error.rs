#[derive(Debug)]
pub enum CommandType {
    ParseFile,
    ParseStdin,
}

#[derive(Debug)]
pub struct CommandError {
    pub command_type: CommandType,
    pub error: String,
}