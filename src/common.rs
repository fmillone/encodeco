pub trait Codec {
    fn encode(input: &str) -> Result<String, String>;
    fn decode(input: &str) -> Result<String, String>;
}

pub trait WithCommand {
    fn command() -> clap::Command;
    fn process(matches: &clap::ArgMatches) -> Result<(), String>;
}
