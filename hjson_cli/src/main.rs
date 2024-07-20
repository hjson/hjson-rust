extern crate core;
extern crate docopt;
extern crate serde;
extern crate serde_hjson;
extern crate serde_json;

use docopt::Docopt;
use serde_hjson::Value;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Hjson, the Human JSON.

Usage:
  hjson [options]
  hjson [options] <input>
  hjson (-h | --help)
  hjson (-V | --version)

Options:
  -h --help     Show this screen.
  -j            Output as formatted JSON.
  -c            Output as JSON.
  -V --version  Show version.
";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        println!("Hjson CLI {}", VERSION);
        return;
    }

    let input = args.get_str("<input>");
    let mut buffer = String::new();

    if input != "" {
        let mut f = File::open(&Path::new(input)).unwrap();
        f.read_to_string(&mut buffer).unwrap();
    } else {
        io::stdin().read_to_string(&mut buffer).unwrap();
    }

    let data: Value = serde_hjson::from_str(&buffer).unwrap();

    if args.get_bool("-j") {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    } else if args.get_bool("-c") {
        println!("{}", serde_json::to_string(&data).unwrap());
    } else {
        println!("{}", serde_hjson::to_string(&data).unwrap());
    }
}
