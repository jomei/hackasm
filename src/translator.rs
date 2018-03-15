use line::Line;
use std::collections::HashMap;

lazy_static! {
    static ref JUMP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("null", "000");
        map.insert("JGT", "001");
        map.insert("JEQ", "010");
        map.insert("JGE", "011");
        map.insert("JLT", "100");
        map.insert("JNE", "101");
        map.insert("JLE", "110");
        map.insert("JMP", "111");
        map
    };

    static ref DEST: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("null", "000");
        map.insert("M",   "001");
        map.insert("D",   "010");
        map.insert("MD",  "011");
        map.insert("A",   "100");
        map.insert("AM",  "101");
        map.insert("AD",  "110");
        map.insert("AMD", "111");
        map
    };

    static ref COMP0: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("0", "101010");
        map.insert("1", "111111");
        map.insert("-1", "111010");
        map.insert("D", "001100");
        map.insert("!D", "001101");
        map.insert("!A", "110001");
        map.insert("-D", "001111");
        map.insert("-A", "110011");
        map.insert("D+1", "011111");
        map.insert("A+1", "110111");
        map.insert("D-1", "001110");
        map.insert("A-1", "110010");
        map.insert("D+A", "000010");
        map.insert("D-A", "010011");
        map.insert("A-D", "000111");
        map.insert("D&A", "000000");
        map.insert("D|A", "010101");
        map
    };

    static ref COMP1: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("M", "110000");
        map.insert("!M", "110001");
        map.insert("-M", "110011");
        map.insert("M+1", "110111");
        map.insert("M-1", "110010");
        map.insert("D+M", "000010");
        map.insert("D-M", "010011");
        map.insert("D&M", "000000");
        map.insert("D|M", "010101");
        map
    };
}

pub fn call(line: Line, symbol_table: &HashMap<String, usize>) -> String {
    if line.is_a() {
       return translate_a(line, symbol_table)
    } else {
        return translate_c(line, symbol_table)
    }
}

fn translate_a(line: Line, symbol_table: &HashMap<String, usize>) -> String {
    let value = symbol_table.get(&line.symbol().unwrap()).expect("symbol not found");
    let binary = format!("{:016b}", value);
    return format!("@{}", binary)
}

fn translate_c(line: Line, symbol_table: &HashMap<String, usize>) -> String {
    let inner = line.inner;
    let first_split:Vec<&str> = inner.split("=").collect();
    let second_split:Vec<&str> = first_split[1].split(";").collect();

    let mut  a_bit;
    let mut comp;

    if COMP0.contains_key(second_split[0]) {
        a_bit = "0";
        comp = COMP0.get(second_split[0]).expect("Unexpected operand");
    } else {
        a_bit = "1";
        comp = COMP1.get(second_split[0]).expect("Unexpected operand");
    }

    let dest = DEST.get(first_split[0]).expect("Unexpected operand");
    let jump = JUMP.get(second_split[1]).expect("Unexpected operand");

    return format!("111{}{}{}{}", a_bit, comp, dest, jump);
}