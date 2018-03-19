mod instruction;
mod reader;
mod parser;
mod symbol_table;
mod translator;


#[macro_use]
extern crate lazy_static;

use std::io::Read;
use symbol_table::Builder;
use instruction::Instruction;


pub struct HackAsm {}

impl HackAsm {
     pub fn compile<R: Read>(content: R) -> Vec<String> {
         let lines = reader::call(content);

         let instructions: Vec<Instruction> = parser::call(lines);

         let table = Builder::new().call(&instructions);
         println!("Instructions read: {}", instructions.len());
         let translated:Vec<String> = instructions
             .iter()
             .filter(|l| !l.is_label())
             .map(|l| translator::call(l, &table).to_string())
             .map(|l| {println!("{}",  l); l})
             .collect();
         println!("Instructions translated: {}", translated.len());
         translated
    }
}