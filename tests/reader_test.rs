extern crate hackasm;

use hackasm::reader;
use hackasm::line::Line;
use std::fs::File;

#[test]
fn read() {
    let file_path = "tests/fixtures/test.asm";
    let file = File::open(&file_path)
        .expect(&format!("Can't open the {} file", &file_path));
    let lines:Vec<Line> = reader::call(file);
    assert_eq!(lines.len(), 9)
}