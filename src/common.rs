use std::io;

pub trait Codec {
    fn encode(input: &str) -> Result<String, String>;
    fn decode(input: &str) -> Result<String, String>;
}

pub trait WithCommand {
    fn command() -> clap::Command;
    fn process(matches: &clap::ArgMatches) -> Result<(), String>;
}

pub fn get_input(arg_input: Option<&String>) -> io::Result<String> {
    match arg_input {
        Some(input) => Ok(input.clone()),
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            buffer = buffer.trim().to_string();
            if buffer.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No input provided",
                ))
            } else {
                Ok(buffer)
            }
        }
    }
}
