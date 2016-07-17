
extern crate num;
extern crate core;
extern crate docopt;
extern crate serde;
extern crate serde_hjson;
extern crate serde_json;

use serde_hjson::Value;
use docopt::Docopt;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io;

const USAGE: &'static str = "
Hjson, the Human JSON.

Usage:
  hjson [options]
  hjson [options] <input>
  hjson (-h | --help)
  hjson --version

Options:
  -h --help     Show this screen.
  -j            Output as formatted JSON.
  -c            Output as JSON.
  --version     Show version.
";

fn main() {


    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let input = args.get_str("<input>");
    let mut buffer = String::new();

    if input != "" {
        let mut f = File::open(&Path::new(input)).unwrap();
        f.read_to_string(&mut buffer).unwrap();
    } else {
        io::stdin().read_to_string(&mut buffer).unwrap();
    }

    let data : Value = serde_hjson::from_str(&buffer).unwrap();

    if args.get_bool("-j") {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    } else if args.get_bool("-c") {
        println!("{}", serde_json::to_string(&data).unwrap());
    } else {
        println!("{}", serde_hjson::to_string(&data).unwrap());
    }
}
