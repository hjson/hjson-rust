extern crate clap;
extern crate core;
extern crate serde;
extern crate serde_hjson;
extern crate serde_json;

use clap::Parser;
use serde_hjson::Value;

use std::fs::File;
use std::io;
use std::io::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::Parser, Clone, Debug)]
#[group(id = "formatting", required = false, multiple = false)]
/// Hjson, the Human JSON.
pub struct HJson {
    /// Output as formatted json
    #[arg(short = 'j', action, group = "formatting")]
    as_formatted_json: bool,

    /// Output as json
    #[arg(short = 'c', action, group = "formatting")]
    as_json: bool,

    /// If specified, read from this file, otherwise read from stdin
    input: Option<std::path::PathBuf>,

    /// Show version
    #[arg(long, short = 'V', action)]
    version: bool,
}

fn main() {
    let args = HJson::parse();
    if args.version {
        println!("Hjson CLI {}", VERSION);
        return;
    }
    let mut buffer = String::new();

    if let Some(input) = args.input {
        let mut f = File::open(input).unwrap();
        f.read_to_string(&mut buffer).unwrap();
    } else {
        io::stdin().read_to_string(&mut buffer).unwrap();
    }

    let data: Value = serde_hjson::from_str(&buffer).unwrap();

    if args.as_formatted_json {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    } else if args.as_json {
        println!("{}", serde_json::to_string(&data).unwrap());
    } else {
        println!("{}", serde_hjson::to_string(&data).unwrap());
    }
}
