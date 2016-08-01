
extern crate serde;
extern crate serde_hjson;
extern crate serde_json;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use serde_hjson::Value;

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

fn get_result_content(name: &str) -> io::Result<(String,String)> {
    let p1 = format!("./assets/{}_result.json", name);
    let p2 = format!("./assets/{}_result.hjson", name);
    Ok(( try!(get_content(&p1)), try!(get_content(&p2))))
}

macro_rules! run_test {
    // {{ is a workaround for rust stable
    ($v: ident, $list: expr, $fix: expr) => {{
        let name = stringify!($v);
        $list.push(format!("{}_test", name));
        println!("- running {}", name);
        let should_fail = name.starts_with("fail");
        let test_content = get_test_content(name).unwrap();
        let data : serde_hjson::Result<Value> = serde_hjson::from_str(&test_content);
        assert!(should_fail == data.is_err());

        if !should_fail {
            let udata = data.unwrap();
            //let (mut rjson, mut rhjson) = get_result_content(name).unwrap();
            let ( rjson, rhjson) = get_result_content(name).unwrap();
            let actual_hjson = serde_hjson::to_string(&udata).unwrap();
            let actual_json = $fix(serde_json::to_string_pretty(&udata).unwrap());
            // if !$exact {
            //     let rjson_v: Value = serde_hjson::from_str(&rjson).unwrap();
            //     rjson = serde_hjson::to_string_pretty(&rjson_v).unwrap();
            // }
            if rhjson != actual_hjson {
                println!("{:?}\n---hjson expected\n{}\n---hjson actual\n{}\n---\n", name, rhjson, actual_hjson);
            }
            if rjson != actual_json {
                println!("{:?}\n---json expected\n{}\n---json actual\n{}\n---\n", name, rjson, actual_json);
            }
            assert!(rhjson == actual_hjson && rjson == actual_json);
        }
    }}
}

// add fixes where rust's json differs from javascript

fn no_fix(json: String) -> String { json }

fn fix_kan(json: String) -> String { json.replace("    -0,", "    0,") }

fn fix_pass1(json: String) -> String {
    json
    .replace("0.000000000000123456789,", "1.23456789e-13,")
    .replace("12345678900000000000000000000000000,", "1.23456789e+34,")
    .replace("23456789012000000000000000000000000000000000000000000000000000000000000000000,", "2.3456789012e+76,")
}

#[test]
fn test_hjson() {

    let mut done : Vec<String> = Vec::new();

    println!("");
    run_test!(charset, done, no_fix);
    run_test!(comments, done, no_fix);
    run_test!(empty, done, no_fix);
    run_test!(fail10, done, no_fix);
    run_test!(fail11, done, no_fix);
    run_test!(fail12, done, no_fix);
    run_test!(fail13, done, no_fix);
    run_test!(fail14, done, no_fix);
    run_test!(fail15, done, no_fix);
    run_test!(fail16, done, no_fix);
    run_test!(fail17, done, no_fix);
    run_test!(fail19, done, no_fix);
    run_test!(fail20, done, no_fix);
    run_test!(fail21, done, no_fix);
    run_test!(fail22, done, no_fix);
    run_test!(fail23, done, no_fix);
    run_test!(fail24, done, no_fix);
    run_test!(fail26, done, no_fix);
    run_test!(fail28, done, no_fix);
    run_test!(fail29, done, no_fix);
    run_test!(fail2, done, no_fix);
    run_test!(fail30, done, no_fix);
    run_test!(fail31, done, no_fix);
    run_test!(fail32, done, no_fix);
    run_test!(fail33, done, no_fix);
    run_test!(fail34, done, no_fix);
    run_test!(fail5, done, no_fix);
    run_test!(fail6, done, no_fix);
    run_test!(fail7, done, no_fix);
    run_test!(fail8, done, no_fix);
    run_test!(failKey1, done, no_fix);
    run_test!(failKey2, done, no_fix);
    run_test!(failKey3, done, no_fix);
    run_test!(failObj1, done, no_fix);
    run_test!(failObj2, done, no_fix);
    run_test!(failObj3, done, no_fix);
    run_test!(failStr1, done, no_fix);
    run_test!(failStr2, done, no_fix);
    run_test!(failStr3, done, no_fix);
    run_test!(failStr4, done, no_fix);
    run_test!(failStr5, done, no_fix);
    run_test!(failStr6, done, no_fix);
    run_test!(kan, done, fix_kan);
    run_test!(keys, done, no_fix);
    run_test!(oa, done, no_fix);
    run_test!(pass1, done, fix_pass1);
    run_test!(pass2, done, no_fix);
    run_test!(pass3, done, no_fix);
    run_test!(pass4, done, no_fix);
    run_test!(passSingle, done, no_fix);
    run_test!(root, done, no_fix);
    run_test!(stringify1, done, no_fix);
    run_test!(strings, done, no_fix);
    run_test!(trail, done, no_fix);

    // check if we include all assets
    let paths = fs::read_dir("./assets/").unwrap();

    let all = paths.map(|item| {
        String::from(item.unwrap().path().file_stem().unwrap().to_str().unwrap())
    })
    .filter(|x| x.contains("_test"))
    .collect::<Vec<String>>();

    let missing = all.into_iter().filter(|x| done.iter().find(|y| &x == y) == None).collect::<Vec<String>>();

    if missing.len() > 0 {
        for item in missing {
            println!("missing: {}", item);
        }
        assert!(false);
    }
}
