mod line;
mod reader;
pub mod symbol_table;

#[macro_use]
extern crate lazy_static;

use std::io::Read;


pub struct HackAsm {}

impl HackAsm {
    pub fn compile<R: Read>(content: R) {

    }
}