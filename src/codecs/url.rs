use clap::{Arg, Command};
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};

use crate::common::{get_input, Codec, WithCommand};

pub struct UrlCodec {}

impl Codec for UrlCodec {
    fn encode(input: &str) -> Result<String, String> {
        Ok(utf8_percent_encode(input, NON_ALPHANUMERIC).to_string())
    }

    fn decode(input: &str) -> Result<String, String> {
        percent_decode(input.as_bytes())
            .decode_utf8()
            .map(|s| s.to_string())
            .map_err(|e| e.to_string())
    }
}

impl WithCommand for UrlCodec {
    fn command() -> clap::Command {
        Command::new("url")
            .alias("u")
            .about("url encode/decode")
            .subcommand(
                Command::new("encode")
                    .alias("e")
                    .about("url encode")
                    .arg(Arg::new("input").help("input to encode")),
            )
            .subcommand(
                Command::new("decode")
                    .about("url decode")
                    .alias("d")
                    .arg(Arg::new("input").help("input to decode")),
            )
    }

    fn process(matches: &clap::ArgMatches) -> Result<(), String> {
        let result = match matches.subcommand() {
            Some(("encode", sub_matches)) => {
                let input = get_input(sub_matches.get_one::<String>("input")).unwrap();
                UrlCodec::encode(&input)
            }
            Some(("decode", sub_matches)) => {
                let input = get_input(sub_matches.get_one::<String>("input")).unwrap();
                UrlCodec::decode(&input)
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
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        assert_eq!(
            UrlCodec::encode("Hello, World!").unwrap(),
            "Hello%2C%20World%21"
        );
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(
            UrlCodec::decode("Hello%2C%20World%21").unwrap(),
            "Hello, World!"
        );
    }

    #[test]
    fn test_url_encode_decode() {
        let input = "Hello, World!";
        let encoded = UrlCodec::encode(input).unwrap();
        let decoded = UrlCodec::decode(&encoded).unwrap();
        assert_eq!(decoded, input);
    }
}
