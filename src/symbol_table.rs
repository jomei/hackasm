use std::collections::HashMap;
use line::Line;

const KNOWN_SYMBOLS: HashMap<&str, i32> =  [
    ("R0", 0), ("R1", 1),("R2", 2), ("R3", 3), ("R4", 4), ("R5", 5),
    ("R6", 6), ("R7", 7),("R8", 8), ("R9", 9), ("R10", 10), ("R11", 11),
    ("R12", 12), ("R13", 13),("R14", 14), ("R15", 15),
    ("SCREEN", 16384), ("KBD", 24576),("SP", 0), ("LCL", 1),
    ("ARG", 2), ("THIS", 3),("THAT", 4)
].iter().collect();

pub struct Builder {
    start_memory: u32,
    counter: u32
}

impl Builder {
    pub fn new(start_memory: u32) {
        let start_memory = start_memory || 1024;
        Builder { start_memory, counter: 0}
    }

    pub fn call(lines: Vec<Line>) -> HashMap<String, u32> {
        let result = KNOWN_SYMBOLS.clone();
        for (i, line) in lines.iter().enumerate() {
            if line.is_symbol() && !result.containts_key(line.symbol()) {
                result.insert(line.symbol(), gen_value(line) )
            }
        }

        return result;
    }

    fn gen_value(line: Line) -> u32 {
        if line.is_a() {
            return gen_a()
        } else {
            return line.number + 1
        }
    }

    fn gen_a(&mut self) -> u32 {
        let a = self.start_memory + self.counter;
        self.counter += 1;
        return a
    }
}