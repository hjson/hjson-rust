//! # What is Hjson?
//!
//! A configuration file format for humans. Relaxed syntax, fewer mistakes, more comments.
//! See https://hjson.github.io
//!
//! Data types that can be encoded are JavaScript types (see the `serde_hjson:Value` enum for more
//! details):
//!
//! * `Boolean`: equivalent to rust's `bool`
//! * `I64`: equivalent to rust's `i64`
//! * `U64`: equivalent to rust's `u64`
//! * `F64`: equivalent to rust's `f64`
//! * `String`: equivalent to rust's `String`
//! * `Array`: equivalent to rust's `Vec<T>`, but also allowing objects of different types in the
//!    same array
//! * `Object`: equivalent to rust's `serde_hjson::Map<String, serde_hjson::Value>`
//! * `Null`
//!
//!
//! # Examples of use
//!
//! ## Parsing a `str` to `Value` and reading the result
//!
//! ```rust
//! use serde_hjson::Value;
//!
//! fn main() {
//!     let data: Value = serde_hjson::from_str("{foo: 13, bar: \"baz\"}").unwrap();
//!     println!("data: {:?}", data);
//!     println!("object? {}", data.is_object());
//!
//!     let obj = data.as_object().unwrap();
//!     let foo = obj.get("foo").unwrap();
//!
//!     println!("array? {:?}", foo.as_array());
//!     // array? None
//!     println!("u64? {:?}", foo.as_u64());
//!     // u64? Some(13u64)
//!
//!     for (key, value) in obj.iter() {
//!         println!("{}: {}", key, match *value {
//!             Value::U64(v) => format!("{} (u64)", v),
//!             Value::String(ref v) => format!("{} (string)", v),
//!             _ => unreachable!(),
//!         });
//!     }
//!     // bar: baz (string)
//!     // foo: 13 (u64)
//! }
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

extern crate core;
#[cfg(feature = "preserve_order")]
extern crate linked_hash_map;
extern crate num_traits;
extern crate regex;
extern crate serde;

pub use self::de::{
    from_iter, from_reader, from_slice, from_str, Deserializer, StreamDeserializer,
};
pub use self::error::{Error, ErrorCode, Result};
pub use self::ser::{to_string, to_vec, to_writer, Serializer};
pub use self::value::{from_value, to_value, Map, Value};

pub mod builder;
pub mod de;
pub mod error;
pub mod ser;
mod util;
pub mod value;
