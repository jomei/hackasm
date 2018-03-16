mod line;
mod reader;
mod symbol_table;
mod translator;


#[macro_use]
extern crate lazy_static;

use std::io::Read;
use symbol_table::Builder;


pub struct HackAsm {}

impl HackAsm {
     pub fn compile<R: Read>(content: R) -> Vec<String> {
        let lines = reader::call(content);
        let table = Builder::new().call(&lines);

        lines
            .iter()
            .map(|l| translator::call(l, &table).to_string())
            .collect()
    }
}