extern crate hackasm;

use hackasm::symbol_table::Builder;
use hackasm::reader;
use hackasm::line::Line;
use std::fs::File;
use std::collections::HashMap;

#[test]
fn builder() {
    let file_path = "tests/fixtures/test.asm";
    let file = File::open(&file_path)
        .expect(&format!("Can't open the {} file", &file_path));
    let lines:Vec<Line> = reader::call(file);
    let t:HashMap<String, usize> = Builder::new().call(lines);
    assert!(t.contains_key("R1"));
    assert!(t.contains_key("some_var"));
    assert!(t.contains_key("END"));
}
