use line::Line;
use std::collections::HashMap;

lazy_static! {
    static ref JUMP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("", "000");
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
        map.insert("", "000");
        map.insert("0", "000");
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
        map.insert("A", "110000");
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

pub fn call(line: &Line, symbol_table: &HashMap<String, usize>) -> String {
    println!("{}", line.inner);
    if line.is_a() {
       return translate_a(line, symbol_table)
    } else {
        return translate_c(line)
    }
}

fn translate_a(line: &Line, symbol_table: &HashMap<String, usize>) -> String {
    let value = symbol_table.get(&line.symbol().unwrap()).expect("symbol not found");
    let binary = format!("{:016b}", value);
    return binary
}

fn translate_c(line: &Line) -> String {
    let a_bit;
    let comp;
    if COMP0.contains_key::<str>(&line.get_comp()) {
        a_bit = "0";
        comp = COMP0.get::<str>(&line.get_comp())
            .expect(&format!("Unexpected comp operand: {} at line {}", &line.get_comp(), line.number));
    } else {
        a_bit = "1";
        comp = COMP1.get::<str>(&line.get_comp())
            .expect(&format!("Unexpected comp operand: {} at line {}", &line.get_comp(), line.number));
    }

    let dest = DEST.get::<str>(&line.get_dest())
        .expect(&format!("Unexpected dest operand: {} at line {}", &line.get_dest(), line.number));

    let jump = JUMP.get::<str>(&line.get_jump())
        .expect(&format!("Unexpected jump operand: {} at line {}", &line.get_jump(), line.number));

    return format!("111{}{}{}{}", a_bit, comp, dest, jump);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use line::Line;
    use translator;

    fn symbol_table() -> HashMap<String, usize> {
        let mut table = HashMap::new();
        table.insert("some_var".to_string(), 16);
        return table
    }

    #[test]
    fn a_instruction() {
        let line = Line::new("@some_var".to_string(), 0);
        let result = translator::call(&line, &symbol_table());
        assert_eq!("0000000000010000", result)
    }

    #[test]
    fn c_instruction() {
        let line = Line::new("MD=D+1".to_string(), 0);
        let result = translator::call(&line, &symbol_table());
        assert_eq!("1110011111011000", result)
    }

    #[test]
    fn c_instruction_1() {
        let line = Line::new("D=A".to_string(), 0);
        let result = translator::call(&line, &symbol_table());
        assert_eq!("1110011111011000", result)
    }
}