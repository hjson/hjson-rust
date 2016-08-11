
extern crate serde;
extern crate serde_hjson;
extern crate serde_json;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use serde_hjson::Value;
use regex::Regex;

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
            let ( rjson, rhjson) = get_result_content(name).unwrap();
            let actual_hjson = serde_hjson::to_string(&udata).unwrap();
            let actual_json = $fix(serde_json::to_string_pretty(&udata).unwrap());
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

fn std_fix(json: String) -> String {
    // serde_json serializes integers with a superfluous .0 suffix
    let re = Regex::new(r"(?m)(?P<d>\d)\.0(?P<s>,?)$").unwrap();
    re.replace_all(&json, "$d$s")
}

fn fix_kan(json: String) -> String { std_fix(json).replace("    -0,", "    0,") }

fn fix_pass1(json: String) -> String {
    std_fix(json)
    .replace("1.23456789e34", "1.23456789e+34")
    .replace("2.3456789012e76", "2.3456789012e+76")
}

#[test]
fn test_hjson() {

    let mut done : Vec<String> = Vec::new();

    println!("");
    run_test!(charset, done, std_fix);
    run_test!(comments, done, std_fix);
    run_test!(empty, done, std_fix);
    run_test!(fail10, done, std_fix);
    run_test!(fail11, done, std_fix);
    run_test!(fail12, done, std_fix);
    run_test!(fail13, done, std_fix);
    run_test!(fail14, done, std_fix);
    run_test!(fail15, done, std_fix);
    run_test!(fail16, done, std_fix);
    run_test!(fail17, done, std_fix);
    run_test!(fail19, done, std_fix);
    run_test!(fail20, done, std_fix);
    run_test!(fail21, done, std_fix);
    run_test!(fail22, done, std_fix);
    run_test!(fail23, done, std_fix);
    run_test!(fail24, done, std_fix);
    run_test!(fail26, done, std_fix);
    run_test!(fail28, done, std_fix);
    run_test!(fail29, done, std_fix);
    run_test!(fail2, done, std_fix);
    run_test!(fail30, done, std_fix);
    run_test!(fail31, done, std_fix);
    run_test!(fail32, done, std_fix);
    run_test!(fail33, done, std_fix);
    run_test!(fail34, done, std_fix);
    run_test!(fail5, done, std_fix);
    run_test!(fail6, done, std_fix);
    run_test!(fail7, done, std_fix);
    run_test!(fail8, done, std_fix);
    run_test!(failKey1, done, std_fix);
    run_test!(failKey2, done, std_fix);
    run_test!(failKey3, done, std_fix);
    run_test!(failObj1, done, std_fix);
    run_test!(failObj2, done, std_fix);
    run_test!(failObj3, done, std_fix);
    run_test!(failStr1, done, std_fix);
    run_test!(failStr2, done, std_fix);
    run_test!(failStr3, done, std_fix);
    run_test!(failStr4, done, std_fix);
    run_test!(failStr5, done, std_fix);
    run_test!(failStr6, done, std_fix);
    run_test!(kan, done, fix_kan);
    run_test!(keys, done, std_fix);
    run_test!(oa, done, std_fix);
    run_test!(pass1, done, fix_pass1);
    run_test!(pass2, done, std_fix);
    run_test!(pass3, done, std_fix);
    run_test!(pass4, done, std_fix);
    run_test!(passSingle, done, std_fix);
    run_test!(root, done, std_fix);
    run_test!(stringify1, done, std_fix);
    run_test!(strings, done, std_fix);
    run_test!(trail, done, std_fix);

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
