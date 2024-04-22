pub enum BitwiseOperation {
    Unknown,
    LSL,
    LSR,
    ROR,
}

impl BitwiseOperation {
    pub fn from_str(operation: &str) -> Self {
        match operation {
            "LSL" => Self::LSL,
            "LSR" => Self::LSR,
            "ROR" => Self::ROR,
            _ => Self::Unknown,
        }
    }
}

pub fn do_bitwise_operation(value: u64, operation: &str, sh: String) -> u64 {
    let v = sh.parse::<u32>().expect("failed to parse shift value to u32");
    match BitwiseOperation::from_str(operation) {
        BitwiseOperation::LSL => value.wrapping_shl(v),
        BitwiseOperation::LSR => value.wrapping_shr(v),
        BitwiseOperation::ROR => value.rotate_right(v),
        _ => panic!("invalid bitwise operation: {}", operation),
    }
}