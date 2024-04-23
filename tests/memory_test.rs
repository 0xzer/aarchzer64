extern crate aarchzer64;

use aarchzer64::{cpu::{context::CpuContext, registers::Register}, unsafe_util::ptr::PointerExt};

const STR_INSTRUCTION_MODEL: &str = include_str!("./models/str.s");
const LDR_INSTRUCTION_MODEL: &str = include_str!("./models/ldr.s");
const LDRB_INSTRUCTION_MODEL: &str = include_str!("./models/ldrb.s");

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

#[test]
fn ldrb() {
    let mut ctx = CpuContext::new_from_text(1024, LDRB_INSTRUCTION_MODEL);
    match ctx.execute() {
        Ok(_) => {
            assert_eq!(ctx.get_register_value_as::<u8>(Register::X7).unwrap(), 176);
            assert_eq!(ctx.get_register_value_as::<u64>(Register::X6).unwrap(), 734439412276);
        },
        Err(e) => {
            println!("failed to execute ldrb test: {}", e.message);
        }
    }
}

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