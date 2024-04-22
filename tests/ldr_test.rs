extern crate aarchzer64;

use aarchzer64::{cpu::{context::CpuContext, registers::Register}, unsafe_util::ptr::PointerExt};

/*
    mov x1, sp
    mov x2, #5000
    str x2, [x1]
    mov x3, #0x2710
    str x3, [x1, #8]
    mov x0, #15000
    str x0, [x1, #16]
    ldr x4, [x1]
    ldr x5, [x1, #8]
    ldr x6, [x1, #16]
    mov x2, #2
    ldr x7, [x1, x2, LSL #3]
    mov x8, x1
    add x1, x1, x2, LSL #3
    str x1, [x8, #8]
    ldr w10, [x1]
*/
const LDR_INSTRUCTION_MODEL: &str = include_str!("./models/ldr.s");
#[test]
fn ldr() {
    let mut ctx = CpuContext::new_from_text(1024, LDR_INSTRUCTION_MODEL);
    match ctx.execute() {
        Ok(_) => unsafe {
            // x4 should hold 5000
            let x4 = ctx.get_register_value_as::<u64>(Register::X4).unwrap();
            assert_eq!(x4, 5000);
            // x5 should hold 10000
            let x5 = ctx.get_register_value_as::<u64>(Register::X5).unwrap();
            assert_eq!(x5, 10000);
            // x6 should hold 15000
            let x6 = ctx.get_register_value_as::<u64>(Register::X6).unwrap();
            assert_eq!(x6, 15000);
            // x7 should hold 15000
            let x7 = ctx.get_register_value_as::<u64>(Register::X7).unwrap();
            assert_eq!(x7, 15000);
            // x8 should hold the stack pointer
            let x8 = ctx.get_register_value_as::<*mut u8>(Register::X8).unwrap();
            // sp + 16 pointer should be stored at x8+0x8
            let sp = x8.offset(0x8).read_ptr();
            // 15000 was stored at sp + 16 before, therefor sp_16_value should now be 15000 
            let sp_16_value = sp.read_u64();
            assert_eq!(sp_16_value, 15000);
            // x1 points to sp + 16
            // ldr w10, [x1] loads the lower 32-bits of the value which sp + 16 points to into w10.
            // w10 should then be loaded with 15000
            let w10 = ctx.get_register_value_as::<u32>(Register::W10).unwrap();
            assert_eq!(w10, 15000);
        },
        Err(e) => {
            println!("failed to execute ldr test: {}", e.message);
        }
    }
}