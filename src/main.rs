mod writer;

extern crate hackasm;
use hackasm::HackAsm;

use std::env;
use std::fs::File;

use writer::Writer;

fn main() {
    let file_path = env::args().nth(1).expect("No .asm file provided");
    let file = File::open(&file_path)
        .expect(&format!("Can't open the {} file", &file_path));

    let mut writer = Writer::new(file_path);

    HackAsm::compile(file)
        .iter()
        .for_each(|l| writer.call(l.to_string()));
}

