#[macro_use]
extern crate log;

use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};

use crate::parser::parse_file;
use crate::utils::fs::read_file;

mod parser;
mod utils;

fn main() {
    configure_logger();

    let matches = define_cli();

    let file_path = matches.value_of("file").unwrap();
    let file_path_buf = PathBuf::from(file_path);

    // Read file.
    let content = match read_file(&file_path_buf) {
        Ok(v) => v,
        Err(e) => {
            error!("Cannot read file: {}", e);
            return;
        }
    };

    // Parse file.
    let parsed_file = match parse_file(content.as_str(), file_path) {
        Ok(v) => v,
        Err(e) => {
            error!("Parsing error: {:?}", e);
            return;
        }
    };

    println!("{}", parsed_file);
}

fn define_cli() -> ArgMatches {
    App::new("sandc")
        .about("The Sand language compiler")
        .version(clap::crate_version!())
        .arg(
            Arg::new("file")
                .about("the input file")
                .index(1)
                .required(true),
        )
        .get_matches()
}

fn configure_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    pretty_env_logger::init();
}
