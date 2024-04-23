extern crate aarchzer64;

use aarchzer64::cpu::{context::CpuContext, registers::Register};

const ADD_INSTRUCTION_MODEL: &str = include_str!("./models/add.s");
const SUB_INSTRUCTION_MODEL: &str = include_str!("./models/sub.s");

#[test]
fn add() {
    let mut ctx = CpuContext::new_from_text(1024, ADD_INSTRUCTION_MODEL);
    match ctx.execute() {
        Ok(_) => {
            assert_eq!(ctx.get_register_value(Register::X1), 34);
            assert_eq!(ctx.get_register_value(Register::X3), 13);
            assert_eq!(ctx.get_register_value(Register::X5), 16);
            assert_eq!(ctx.get_register_value(Register::W8) as u32, 108);
			assert_eq!(ctx.get_register_value(Register::X18), 12884901888);
            assert_eq!(ctx.get_register_value(Register::X19), 51539607555);
        },
        Err(e) => {
            println!("failed to execute add test: {}", e.message);
        }
    }
}


#[test]
fn sub() {
    let mut ctx = CpuContext::new_from_text(1024, SUB_INSTRUCTION_MODEL);
    match ctx.execute() {
        Ok(_) => {
            assert_eq!(ctx.get_register_value(Register::X1), 2);
            assert_eq!(ctx.get_register_value(Register::X3), 13);
            assert_eq!(ctx.get_register_value(Register::X5), 16);
            assert_eq!(ctx.get_register_value(Register::W8) as u32, 4294967214);
			assert_eq!(ctx.get_register_value(Register::X18), 18446744073709551615);
            assert_eq!(ctx.get_register_value(Register::X19), 281474976710655);
        },
        Err(e) => {
            println!("failed to execute add test: {}", e.message);
        }
    }
}
