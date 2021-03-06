use std::collections::HashMap;
use instruction::Instruction;

lazy_static! {
    static ref KNOWN_SYMBOLS: HashMap<String, usize> = {
        let mut map = HashMap::new();
        map.insert("R0".to_string(), 0);
        map.insert("R1".to_string(), 1);
        map.insert("R2".to_string(), 2);
        map.insert("R3".to_string(), 3);
        map.insert("R4".to_string(), 4);
        map.insert("R5".to_string(), 5);
        map.insert("R6".to_string(), 6);
        map.insert("R7".to_string(), 7);
        map.insert("R8".to_string(), 8);
        map.insert("R9".to_string(), 9);
        map.insert("R10".to_string(), 10);
        map.insert("R11".to_string(), 11);
        map.insert("R12".to_string(), 12);
        map.insert("R13".to_string(), 13);
        map.insert("R14".to_string(), 14);
        map.insert("R15".to_string(), 15);
        map.insert("SCREEN".to_string(), 16384);
        map.insert("KBD".to_string(), 24576);
        map.insert("SP".to_string(), 0);
        map.insert("LCL".to_string(), 1);
        map.insert("ARG".to_string(), 2);
        map.insert("THIS".to_string(), 3);
        map.insert("THAT".to_string(), 4);
        map
    };
}

pub struct Builder {
    start_memory: usize,
    counter: usize
}

impl Builder {
    pub fn new() -> Self {
        Builder { start_memory: 16, counter: 0}
    }

    pub fn call(&mut self, lines: &Vec<Instruction>) -> HashMap<String, usize> {
        let mut result = KNOWN_SYMBOLS.clone();

        let labels:Vec<&Instruction> = lines
            .iter()
            .filter(|inst| inst.is_label())
            .collect();

        for label in labels.iter() {
            println!("add label {}", label.symbol().unwrap());
            result.insert(label.symbol().unwrap(), label.number);
        }

        for line in lines.iter() {
            if line.is_symbol() {
                let sym = line.symbol().unwrap();
                if !result.contains_key(&sym) {
                    println!("add symbol {}", line.symbol().unwrap());
                    result.insert(sym, self.gen_a());
                }
            }

        }

        return result;
    }


    fn gen_a(&mut self) -> usize {
        let a = self.start_memory + self.counter;
        self.counter += 1;
        return a
    }
}