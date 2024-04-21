use crate::cpu::{context::{CpuContext, RegisterType}, register_value::RegisterValue};

pub unsafe fn do_operand_bitwise_operation(
    args: &Vec<String>,
    operand: &mut RegisterValue,
    sh_operation_index: usize,
    sh_operation_index_value: usize,
) {
    if let Some(sh_operation) = args.get(sh_operation_index) {
        let sh_val = args.get(sh_operation_index_value)
            .expect("failed to retrieve bit operation value from instruction")
            .replace("#", "");
        operand.do_bitwise_operation(sh_operation, sh_val)
    };
}

pub fn parse_dst_with_offset(ctx: &mut CpuContext, instruction: &str) -> (String, Option<usize>) {
    let trimmed = instruction.trim_matches(|c| c == '[' || c == ']').trim();
    let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
    let register = parts[0].to_string();
    let mut offset = None;
    for part in parts.iter().skip(1) {
        if part.starts_with("#") {
            offset = part.trim_start_matches('#').parse::<usize>().ok();
        } else if part.starts_with("w") || part.starts_with("x") {
            let reg_type = get_reg_type(part);
            let reg_val = ctx.get_operand_value(&part.to_string(), reg_type);
            offset = Some(reg_val.to_usize());
        } else if part.contains("LSL") {
            let shift_parts: Vec<&str> = part.split("LSL").collect();
            if shift_parts.len() == 2 {
                let shift = shift_parts[1].replace("#", "").parse::<u32>().ok().map(|value| value);
                if let Some(current_offset) = offset {
                    if let Some(shift_val) = shift {
                        offset = Some(current_offset.wrapping_shl(shift_val));
                    }
                }
            }
        }
    }
    (register, offset)
}

pub fn get_reg_type(input: &str) -> RegisterType {
    let mut chars = input.chars();
    match chars.next() {
        Some('x') => RegisterType::Reg64,
        Some('w') => RegisterType::Reg32,
        _ => panic!("failed to retrieve reg type for input: {}", input),
    }
}