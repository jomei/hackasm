/*
* todo:
* 1. +cli
* 2. +read file as array of strings
* 3. +remove all comments and whitespaces
* 4. build symbol table
* 5. translate each string to hack code
* 6. write result into .hack file
*/
extern crate hackasm;
use hackasm::HackAsm;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    let file_path = env::args().nth(1).expect("No .asm file provided");
    let file = File::open(&file_path)
        .expect(&format!("Can't open the {} file", &file_path));

    HackAsm::compile(file);
}

