# hjson-rust for serde

[![Build Status](https://img.shields.io/travis/laktak/hjson-rust.svg?style=flat-square)](http://travis-ci.org/laktak/hjson-rust)
[![crate](https://img.shields.io/crates/v/serde-hjson.svg)](https://crates.io/crates/serde-hjson)

![Hjson Intro](http://hjson.org/hjson1.gif)

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

The Rust implementation of Hjson is based on the [Serde JSON Serialization Library](https://github.com/serde-rs/json). For other platforms see [hjson.org](http://hjson.org).

This crate is a Rust library for parsing and generating Human JSON [Hjson](http://hjson.org). It is built upon [Serde](https://github.com/serde-rs/serde), a high performance generic serialization framework.

# v0.0.1/unreleased

- Parser is working

Todo:

- Serializer
- Unified number type

# Install

This crate works with Cargo and can be found on [crates.io](https://crates.io/crates/serde-hjson) with a `Cargo.toml` like:

```toml
[dependencies]
serde = "*"
serde-hjson = "*"
```

# Usage

```rust
extern crate serde;
extern crate serde_hjson;

use serde_hjson::Map;

fn main() {
    let mut map = Map::new();
    map.insert("x".to_string(), 1.0);
    map.insert("y".to_string(), 2.0);

    let s = serde_hjson::to_string(&map).unwrap();
    assert_eq!(s, "{\"x\":1,\"y\":2}");

    let deserialized_map: Map<String, f64> = serde_hjson::from_str(&s).unwrap();
    assert_eq!(map, deserialized_map);
}
```

