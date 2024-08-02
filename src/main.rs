use clap::Command;
use encodeco::{
    codecs::{base64::Base64Codec, url::UrlCodec},
    common::WithCommand,
};

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = Command::new("Encodeco")
        .version(VERSION)
        .author(AUTHOR)
        .about("A CLI tool thar encodes and decodes text using various methods")
        .subcommand(Base64Codec::command())
        .subcommand(UrlCodec::command())
        .get_matches();

    match matches.subcommand() {
        Some(("base64", b64_matches)) => {
            Base64Codec::process(b64_matches).unwrap();
        }
        Some(("url", url_matches)) => {
            UrlCodec::process(url_matches).unwrap();
        }
        _ => println!("Please use a valid subcommand. Use --help for more information."),
    }
}
