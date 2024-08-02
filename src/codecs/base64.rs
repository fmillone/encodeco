use base64::{engine::general_purpose, Engine};
use clap::{Arg, Command};

use crate::common::{get_input, Codec, WithCommand};

pub struct Base64Codec {}

impl Codec for Base64Codec {
    fn encode(input: &str) -> Result<String, String> {
        Ok(general_purpose::STANDARD.encode(input))
    }

    fn decode(input: &str) -> Result<String, String> {
        general_purpose::STANDARD
            .decode(input)
            .map_err(|e| e.to_string())
            .and_then(|v| String::from_utf8(v).map_err(|e| e.to_string()))
    }
}

impl WithCommand for Base64Codec {
    fn command() -> clap::Command {
        Command::new("base64")
            .alias("b64")
            .about("base64 encode/decode")
            .subcommand(
                Command::new("encode")
                    .alias("e")
                    .about("base64 encode")
                    .arg(Arg::new("input").help("input to encode")),
            )
            .subcommand(
                Command::new("decode")
                    .about("base64 decode")
                    .alias("d")
                    .arg(Arg::new("input").help("input to decode")),
            )
    }

    fn process(matches: &clap::ArgMatches) -> Result<(), String> {
        let result = match matches.subcommand() {
            Some(("encode", sub_matches)) => {
                let input = get_input(sub_matches.get_one::<String>("input")).unwrap();
                Base64Codec::encode(&input)
            }
            Some(("decode", sub_matches)) => {
                let input = get_input(sub_matches.get_one::<String>("input")).unwrap();
                Base64Codec::decode(&input)
            }
            _ => Err("Invalid subcommand".to_string()),
        };
        match result {
            Ok(output) => {
                println!("{}", output);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(
            Base64Codec::encode("Hello, World!").unwrap(),
            "SGVsbG8sIFdvcmxkIQ=="
        );
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(
            Base64Codec::decode("SGVsbG8sIFdvcmxkIQ==").unwrap(),
            "Hello, World!"
        );
    }

    #[test]
    fn test_invalid_base64_input() {
        assert!(Base64Codec::decode("invalid base64!").is_err());
    }

    #[test]
    fn test_base64_encode_decode() {
        let input = "Hello, World!";
        let encoded = Base64Codec::encode(input).unwrap();
        let decoded = Base64Codec::decode(&encoded).unwrap();
        assert_eq!(decoded, input);
    }
}
