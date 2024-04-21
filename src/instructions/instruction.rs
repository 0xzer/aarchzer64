#[derive(Debug, Clone, Copy)]
pub(crate) enum Mnemonic {
    MOV,
    ADD,
    UNSUPPORTED,
}

impl Mnemonic {
    pub fn from_str(mnemonic: &str) -> Self {
        match mnemonic.to_ascii_lowercase().as_str() {
            "mov" => Mnemonic::MOV,
            "add" => Mnemonic::ADD,
            _ => Mnemonic::UNSUPPORTED,
        }
    }
}

#[derive(Debug)]
pub(crate) struct InstructionRaw {
    pub instruction_mnemonic: Mnemonic,
    pub instruction_args: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct Instructions {
    pub raw: Vec<InstructionRaw>,
}

impl Instructions {
    pub fn new_from_text(asm_code: &str) -> Self {
        Self {
            raw: text_to_raw(asm_code),
        }
    }

}

fn text_to_raw(asm_code: &str) -> Vec<InstructionRaw> {
    let instructions: Vec<String> = asm_code.split("\n").map(|line| line.replace("\t", "")).collect();
    let mut raw_instruction: Vec<InstructionRaw> = Vec::new();
    for instruction in instructions {
        raw_instruction.push(parse_instruction_str(instruction));
    };
    raw_instruction
}

fn parse_instruction_str(instruction: String) -> InstructionRaw {
    let instruction_data: Vec<&str> = instruction.split_whitespace().collect();
    let instruction_mnemonic = Mnemonic::from_str(instruction_data.get(0).unwrap());
    let instruction_args: Vec<String> = instruction_data[1..]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    InstructionRaw {
        instruction_mnemonic,
        instruction_args,
    }
}