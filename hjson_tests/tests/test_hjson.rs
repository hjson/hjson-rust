
extern crate serde;
extern crate serde_hjson;

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

macro_rules! run_test {
    ($v: ident, $list: expr, $exact: expr) => {
        let name = stringify!($v);
        $list.push(format!("{}_test", name));
        println!("- running {}", name);
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

#[test]
fn test_hjson() {

    let mut done : Vec<String> = Vec::new();

    println!("");
    run_test!(charset, done, true);
    run_test!(comments, done, true);
    run_test!(empty, done, true);
    run_test!(fail10, done, true);
    run_test!(fail11, done, true);
    run_test!(fail12, done, true);
    run_test!(fail13, done, true);
    run_test!(fail14, done, true);
    run_test!(fail15, done, true);
    run_test!(fail16, done, true);
    run_test!(fail17, done, true);
    run_test!(fail19, done, true);
    run_test!(fail20, done, true);
    run_test!(fail21, done, true);
    run_test!(fail22, done, true);
    run_test!(fail23, done, true);
    run_test!(fail24, done, true);
    run_test!(fail26, done, true);
    run_test!(fail28, done, true);
    run_test!(fail29, done, true);
    run_test!(fail2, done, true);
    run_test!(fail30, done, true);
    run_test!(fail31, done, true);
    run_test!(fail32, done, true);
    run_test!(fail33, done, true);
    run_test!(fail5, done, true);
    run_test!(fail6, done, true);
    run_test!(fail7, done, true);
    run_test!(fail8, done, true);
    run_test!(failKey1, done, true);
    run_test!(failKey2, done, true);
    run_test!(failKey3, done, true);
    run_test!(failObj1, done, true);
    run_test!(failObj2, done, true);
    run_test!(failObj3, done, true);
    run_test!(failStr1, done, true);
    run_test!(failStr2, done, true);
    run_test!(failStr3, done, true);
    run_test!(failStr4, done, true);
    run_test!(failStr5, done, true);
    run_test!(failStr6, done, true);
    //run_test!(kan, done, true);
    run_test!(keys, done, true);
    run_test!(oa, done, true);
    run_test!(pass1, done, false);
    run_test!(pass2, done, true);
    run_test!(pass3, done, true);
    run_test!(pass4, done, true);
    run_test!(passSingle, done, true);
    run_test!(root, done, true);
    run_test!(stringify1, done, true);
    run_test!(strings, done, true);
    run_test!(trail, done, true);

    // todo: check if we include all assets
    let paths = fs::read_dir("./assets/").unwrap();

    let all = paths.map(|item| {
        String::from(item.unwrap().path().file_stem().unwrap().to_str().unwrap())
    })
    .filter(|x| x.contains("_test"))
    .collect::<Vec<String>>();

    let missing = all.into_iter().filter(|x| done.iter().find(|y| &x == y) == None).collect::<Vec<String>>();

    // if missing.len() > 0 {
    //     for item in missing {
    //         println!("missing: {}", item);
    //     }
    //     assert!(false);
    // }
}
