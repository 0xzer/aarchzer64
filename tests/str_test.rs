extern crate aarchzer64;

use aarchzer64::{cpu::{context::CpuContext, registers::Register}, unsafe_util::ptr::PointerExt};

/*
    mov x1, sp
    mov x2, #5000
    str x2, [x1]
    mov x3, #0x2710
    str w3, [x1, #8]
    str x1, [x1, #16]
    mov w4, #15000
    add w4, w4, w3
    mov x2, #0x3
    str w4, [x1, x2, LSL #3]
*/
const STR_INSTRUCTION_MODEL: &str = include_str!("./models/str.s");
#[test]
fn str() {
    let mut ctx = CpuContext::new_from_text(1024, STR_INSTRUCTION_MODEL);
    match ctx.execute() {
        Ok(_) => unsafe {
            let x1 = ctx.get_register_value_as::<*mut u8>(Register::X1).unwrap();
            // x1 should point to the value 5000
            let x1_value = x1.read_u64();
            assert_eq!(x1_value, 5000);
            // x1 + 8 should point to the value 10000
            let x1_8_value = x1.offset(8).read_u32();
            assert_eq!(x1_8_value, 10000);
            // x1 + 16 should point to x1 (sp at the time)
            let x1_16_value_ptr = x1.offset(16).read_ptr();
            // this should point to the value 5000
            let x1_16_value_u64 = x1_16_value_ptr.read_u64();
            assert_eq!(x1_16_value_u64, 5000);
            // x1 + (x2 << 3) should point to 25000
            let x2_value = ctx.get_register_value_as::<u64>(Register::X2).unwrap();
            let x1_x2_shl_3 = x1.offset(x2_value.wrapping_shl(3) as isize).read_u32();
            assert_eq!(x1_x2_shl_3, 25000);
        },
        Err(e) => {
            println!("failed to execute str test: {}", e.message);
        }
    }
}