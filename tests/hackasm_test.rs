extern crate hackasm;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use hackasm::HackAsm;

fn read_file(file_name: &str) -> File {
    let name = format!("tests/fixtures/{}", file_name);
    let mut file_path = env::current_dir().unwrap();
    file_path.push(name);
    File::open(file_path)
        .expect(&format!("Can't open the {} file", file_name))
}

fn compare_lines(file_name: &str) -> Vec<String> {
    let file = read_file(file_name);
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

fn assert_result(vec_result: Vec<String>, vec_compare: Vec<String>) {
    vec_result
        .iter()
        .enumerate()
        .for_each(|(i, line)| {
            assert_eq!(*line, vec_compare[i])
        });
}

fn check_result(vec_result: Vec<String>, compare_name: &str) {
    assert_result(vec_result, compare_lines(compare_name));
}

#[test]
fn add_test() {
    let result: Vec<String> = HackAsm::compile(read_file("Add.asm"));
    check_result(result, "Add.hack");
}

#[test]
fn max_test() {
    let result: Vec<String> = HackAsm::compile(read_file("Max.asm"));
    check_result(result, "Max.hack");
}

#[test]
fn rect_test() {
    let result: Vec<String> = HackAsm::compile(read_file("Rect.asm"));
    check_result(result, "Rect.hack");
}