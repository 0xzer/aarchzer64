extern crate aarchzer64;

use aarchzer64::cpu::{context::CpuContext, registers::Register};

/*
	mov x0, #1
	mov x1, #2
	add x2, x0, x1
	mov x1, #10
	mov x3, 13
	add x1, x2, x1
	mov x5, 16
	mov x19, 18
	add x1, x19, x5
	add x19, x1, x5
	mov w6, #0xd
	mov w7, 0xf
	add w8, w6, w7
	add w8, w8, 10, LSL #3
	add x19, x1, x19, LSL #5
	add x19, x19, x2, ROR #6
	add x19, x2, x19, LSR #24
	mov x18, x19, LSR #2
*/
const ADD_INSTRUCTION_MODEL: &str = include_str!("./models/add.s");

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