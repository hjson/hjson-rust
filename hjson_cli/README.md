# Hjson CLI

[![Build Status](https://github.com/hjson/hjson-rust/workflows/test/badge.svg)](https://github.com/hjson/hjson-rust/actions)
[![crate](https://img.shields.io/crates/v/serde-hjson.svg?style=flat-square)](https://crates.io/crates/hjson)

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

## Install

Install with `cargo install hjson`

## Usage

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
