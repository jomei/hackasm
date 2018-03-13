/*
* todo:
* 1. +cli
* 2. +read file as array of strings
* 3. +remove all comments and whitespaces
* 4. build symbol table
* 5. translate each string to hack code
* 6. write result into .hack file
*/
mod line;
mod reader;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file_path = env::args().nth(1).expect("No .asm file provided");
    let file = File::open(&file_path)
        .expect(&format!("Can't open the {} file", &file_path));

    let lines = reader::call(file);
//    let symbol_table = table_builder::call(lines);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
