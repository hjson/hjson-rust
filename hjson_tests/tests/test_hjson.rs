
use std::path::Path;
use std::fs::File;
//use std::fs;
use std::io;
use std::io::prelude::*;

extern crate serde;
extern crate serde_hjson;

use serde_hjson::Value;

/*
#[test]
fn test_check_complete() {

    // todo: check if we include all assets
    let paths = fs::read_dir("./assets/").unwrap();
    for path in paths {
        //println!("Name: {}", path.unwrap().path().display())
    }
}
*/

fn get_content(name: &str) -> io::Result<String> {
    let mut f = try!(File::open(&Path::new(name)));
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));
    Ok(buffer)
}

fn get_test_content(name: &str) -> io::Result<String> {
    let mut p = format!("./assets/{}_test.hjson", name);
    if !Path::new(&p).exists() { p = format!("./assets/{}_test.json", name); }
    get_content(&p)
}

fn get_result_content(name: &str) -> io::Result<(String,Option<String>)> {
    let p1 = format!("./assets/{}_result.json", name);
    let p2 = format!("./assets/{}_result.hjson", name);
    Ok(( try!(get_content(&p1)), if !Path::new(&p2).exists() { Some(try!(get_content(&p2))) } else { None }))
}

#[macro_export]
macro_rules! tryfail {
    ($expr:expr) => (match $expr {
        result::Result::Ok(val) => val,
        result::Result::Err(_) => panic!(stringify!($expr))
    })
}

macro_rules! foo {
    ($v: ident, $exact: expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $v() {
            let name = stringify!($v);
            println!("{:?}", name);
            let should_fail = name.starts_with("fail");
            let test_content = get_test_content(name).unwrap();
            let data : serde_hjson::Result<Value> = serde_hjson::from_str(&test_content);
            assert!(should_fail == data.is_err());

            if !should_fail {
                let (mut rjson, _rhjson) = get_result_content(name).unwrap();
                let actual_json = serde_hjson::to_string_pretty(&data.unwrap()).unwrap();
                if !$exact {
                    let rjson_v: Value = serde_hjson::from_str(&rjson).unwrap();
                    rjson = serde_hjson::to_string_pretty(&rjson_v).unwrap();
                }
                if rjson != actual_json {
                    println!("{:?}\n---json expected\n{}\n---json actual\n{}\n---\n", name, rjson, actual_json);
                }
                assert!(rjson == actual_json);
            }
        }
    }
}

foo!(charset, true);
foo!(comments, true);
foo!(empty, true);
foo!(fail10, true);
foo!(fail11, true);
foo!(fail12, true);
foo!(fail13, true);
foo!(fail14, true);
foo!(fail15, true);
foo!(fail16, true);
foo!(fail17, true);
foo!(fail19, true);
foo!(fail20, true);
foo!(fail21, true);
foo!(fail22, true);
foo!(fail23, true);
foo!(fail24, true);
foo!(fail26, true);
foo!(fail28, true);
foo!(fail29, true);
foo!(fail2, true);
foo!(fail30, true);
foo!(fail31, true);
foo!(fail32, true);
foo!(fail33, true);
foo!(fail5, true);
foo!(fail6, true);
foo!(fail7, true);
foo!(fail8, true);
foo!(failKey1, true);
foo!(failKey2, true);
foo!(failKey3, true);
foo!(failObj1, true);
foo!(failObj2, true);
foo!(failObj3, true);
foo!(failStr1, true);
foo!(failStr2, true);
foo!(failStr3, true);
foo!(failStr4, true);
foo!(failStr5, true);
foo!(failStr6, true);
foo!(kan, false);
foo!(keys, true);
foo!(oa, true);
foo!(pass1, false);
foo!(pass2, true);
foo!(pass3, true);
foo!(pass4, true);
foo!(passSingle, true);
foo!(root, true);
foo!(stringify1, true);
foo!(strings, true);
foo!(trail, true);
