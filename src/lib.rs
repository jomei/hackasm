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
         println!("Lines read: {}", lines.len());
         let translated:Vec<String> = lines
             .iter()
             .map(|l| translator::call(l, &table).to_string())
             .map(|l| {println!("{}",  l); l})
             .collect();
         println!("Lines translated: {}", translated.len());
         translated
    }
}