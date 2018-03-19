use instruction::Instruction;

pub fn call(lines: Vec<String>) -> Vec<Instruction> {
    let mut label_amount = 0;

   lines
        .iter()
        .map(|l| Instruction::new(l.to_string(), 0))
        .enumerate()
        .map(|(i, instruction)| {
            let instruction_with_number =
                Instruction::new(instruction.inner, i - label_amount);

            if instruction_with_number.is_label() { label_amount += 1 }

            instruction_with_number
        })
        .collect()
}