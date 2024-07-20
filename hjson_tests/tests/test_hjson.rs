extern crate serde;
extern crate serde_hjson;
extern crate serde_json;

use regex::Regex;
use serde_hjson::Value;
use std::borrow::Cow;
use std::fs;
use std::io;
use std::path::Path;

fn get_test_content(name: &str) -> io::Result<String> {
    let mut p = format!("./assets/{}_test.hjson", name);
    if !Path::new(&p).exists() {
        p = format!("./assets/{}_test.json", name);
    }
    fs::read_to_string(&p)
}

fn get_result_content(name: &str) -> io::Result<(String, String)> {
    let p1 = format!("./assets/{}_result.json", name);
    let p2 = format!("./assets/{}_result.hjson", name);
    Ok((fs::read_to_string(&p1)?, fs::read_to_string(&p2)?))
}

macro_rules! run_test {
    // {{ is a workaround for rust stable
    ($v: ident, $list: expr, $fix: expr) => {{
        let name = stringify!($v);
        $list.push(format!("{}_test", name));
        println!("- running {}", name);
        let should_fail = name.starts_with("fail");
        let test_content = get_test_content(name).unwrap();
        let data: serde_hjson::Result<Value> = serde_hjson::from_str(&test_content);
        assert!(should_fail == data.is_err());

        if !should_fail {
            let udata = data.unwrap();
            let (rjson, rhjson) = get_result_content(name).unwrap();
            let actual_hjson = serde_hjson::to_string(&udata).unwrap();
            let actual_json = serde_json::to_string_pretty(&udata).unwrap();
            let actual_json = $fix(&actual_json);
            if rhjson != actual_hjson {
                println!(
                    "{:?}\n---hjson expected\n{}\n---hjson actual\n{}\n---\n",
                    name, rhjson, actual_hjson
                );
            }
            if rjson != actual_json {
                println!(
                    "{:?}\n---json expected\n{}\n---json actual\n{}\n---\n",
                    name, rjson, actual_json
                );
            }
            assert!(rhjson == actual_hjson && rjson == actual_json);
        }
    }};
}

// add fixes where rust's json differs from javascript

fn std_fix(json: &str) -> Cow<str> {
    // serde_json serializes integers with a superfluous .0 suffix
    let re = Regex::new(r"(?m)(?P<d>\d)\.0(?P<s>,?)$").unwrap();
    re.replace_all(&json, "$d$s")
}

fn fix_kan(json: &str) -> String {
    std_fix(json).replace("    -0,", "    0,")
}

fn fix_pass1(json: &str) -> String {
    std_fix(json)
        .replace("1.23456789e34", "1.23456789e+34")
        .replace("2.3456789012e76", "2.3456789012e+76")
}

#[test]
fn test_hjson() {
    let mut done: Vec<String> = Vec::new();

    println!("");
    run_test!(charset, done, std_fix);
    run_test!(comments, done, std_fix);
    run_test!(empty, done, std_fix);
    run_test!(failCharset1, done, std_fix);
    run_test!(failJSON02, done, std_fix);
    run_test!(failJSON05, done, std_fix);
    run_test!(failJSON06, done, std_fix);
    run_test!(failJSON07, done, std_fix);
    run_test!(failJSON08, done, std_fix);
    run_test!(failJSON10, done, std_fix);
    run_test!(failJSON11, done, std_fix);
    run_test!(failJSON12, done, std_fix);
    run_test!(failJSON13, done, std_fix);
    run_test!(failJSON14, done, std_fix);
    run_test!(failJSON15, done, std_fix);
    run_test!(failJSON16, done, std_fix);
    run_test!(failJSON17, done, std_fix);
    run_test!(failJSON19, done, std_fix);
    run_test!(failJSON20, done, std_fix);
    run_test!(failJSON21, done, std_fix);
    run_test!(failJSON22, done, std_fix);
    run_test!(failJSON23, done, std_fix);
    run_test!(failJSON24, done, std_fix);
    run_test!(failJSON26, done, std_fix);
    run_test!(failJSON28, done, std_fix);
    run_test!(failJSON29, done, std_fix);
    run_test!(failJSON30, done, std_fix);
    run_test!(failJSON31, done, std_fix);
    run_test!(failJSON32, done, std_fix);
    run_test!(failJSON33, done, std_fix);
    run_test!(failJSON34, done, std_fix);
    run_test!(failKey1, done, std_fix);
    run_test!(failKey2, done, std_fix);
    run_test!(failKey3, done, std_fix);
    run_test!(failKey4, done, std_fix);
    run_test!(failMLStr1, done, std_fix);
    run_test!(failObj1, done, std_fix);
    run_test!(failObj2, done, std_fix);
    run_test!(failObj3, done, std_fix);
    run_test!(failStr1a, done, std_fix);
    run_test!(failStr1b, done, std_fix);
    run_test!(failStr1c, done, std_fix);
    run_test!(failStr1d, done, std_fix);
    run_test!(failStr2a, done, std_fix);
    run_test!(failStr2b, done, std_fix);
    run_test!(failStr2c, done, std_fix);
    run_test!(failStr2d, done, std_fix);
    run_test!(failStr3a, done, std_fix);
    run_test!(failStr3b, done, std_fix);
    run_test!(failStr3c, done, std_fix);
    run_test!(failStr3d, done, std_fix);
    run_test!(failStr4a, done, std_fix);
    run_test!(failStr4b, done, std_fix);
    run_test!(failStr4c, done, std_fix);
    run_test!(failStr4d, done, std_fix);
    run_test!(failStr5a, done, std_fix);
    run_test!(failStr5b, done, std_fix);
    run_test!(failStr5c, done, std_fix);
    run_test!(failStr5d, done, std_fix);
    run_test!(failStr6a, done, std_fix);
    run_test!(failStr6b, done, std_fix);
    run_test!(failStr6c, done, std_fix);
    run_test!(failStr6d, done, std_fix);
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

    let all = paths
        .map(|item| String::from(item.unwrap().path().file_stem().unwrap().to_str().unwrap()))
        .filter(|x| x.contains("_test"))
        .collect::<Vec<String>>();

    let missing = all
        .into_iter()
        .filter(|x| done.iter().find(|y| &x == y) == None)
        .collect::<Vec<String>>();

    if missing.len() > 0 {
        for item in missing {
            println!("missing: {}", item);
        }
        assert!(false);
    }
}
