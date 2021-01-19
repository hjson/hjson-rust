#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]

extern crate regex;
extern crate serde;
extern crate serde_hjson;
extern crate skeptic;

mod test_hjson;

#[cfg(feature = "nightly-testing")]
mod skeptic_tests {
    #![cfg_attr(feature = "nightly-testing", allow(toplevel_ref_arg))]
    #![cfg_attr(feature = "nightly-testing", allow(useless_format))]

    include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
}
