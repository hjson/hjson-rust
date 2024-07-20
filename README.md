# hjson-rust for serde

[![Build Status](https://img.shields.io/travis/hjson/hjson-rust.svg?style=flat-square)](http://travis-ci.org/hjson/hjson-rust)
[![crate](https://img.shields.io/crates/v/serde-hjson.svg?style=flat-square)](https://crates.io/crates/serde-hjson)

![Hjson Intro](https://hjson.github.io/hjson1.gif)

```
{
  # specify rate in requests/second (because comments are helpful!)
  rate: 1000

  // prefer c-style comments?
  /* feeling old fashioned? */

  # did you notice that rate doesn't need quotes?
  hey: look ma, no quotes for strings either!

  # best of all
  notice: []
  anything: ?

  # yes, commas are optional!
}
```

The Rust implementation of Hjson is based on the [Serde JSON Serialization Library](https://github.com/serde-rs/json). For other platforms see [hjson.github.io](https://hjson.github.io).

This crate is a Rust library for parsing and generating Human JSON [Hjson](https://hjson.github.io). It is built upon [Serde](https://github.com/serde-rs/serde), a high performance generic serialization framework.

# Install

This crate works with Cargo and can be found on [crates.io](https://crates.io/crates/serde-hjson) with a `Cargo.toml` like:

```toml
[dependencies]
serde = "*"
serde-hjson = "*"
```

## From the Commandline

Install with `cargo install hjson`

```
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
```

Sample:
- run `hjson test.json > test.hjson` to convert to Hjson
- run `hjson -j test.hjson > test.json` to convert to JSON


# Usage

```rust
extern crate serde;
extern crate serde_hjson;

use serde_hjson::{Map,Value};

fn main() {

    // Now let's look at decoding Hjson data

    let sample_text=r#"
    {
        // specify rate in requests/second
        rate: 1000
        array:
        [
            foo
            bar
        ]
    }"#;

    // Decode and unwrap.
    let mut sample: Map<String, Value> = serde_hjson::from_str(&sample_text).unwrap();

    // scope to control lifetime of borrow
    {
        // Extract the rate
        let rate = sample.get("rate").unwrap().as_f64().unwrap();
        println!("rate: {}", rate);

        // Extract the array
        let array : &mut Vec<Value> = sample.get_mut("array").unwrap().as_array_mut().unwrap();
        println!("first: {}", array.get(0).unwrap());

        // Add a value
        array.push(Value::String("tak".to_string()));
    }

    // Encode to Hjson
    let sample2 = serde_hjson::to_string(&sample).unwrap();
    println!("Hjson:\n{}", sample2);
}
```

# API

[see Rust doc](http://hjson.github.io/hjson-rust/serde_hjson/)

# History

[see history.md](history.md)
