use crate::{cpu::{context::{CpuContext, RegisterType}, registers::Register}, unsafe_util::ptr::PointerExt};
use crate::instructions::common::{do_operand_bitwise_operation, get_reg_type, parse_reg_operand_with_offset};

pub unsafe fn mov(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let mut mov_op = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    mov_op = do_operand_bitwise_operation(&args, mov_op, 2, 3);
    ctx.set_register_value(mov_op, dst_index);
}

pub unsafe fn add(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let add_op1 = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let mut add_op2 = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    add_op2 = do_operand_bitwise_operation(&args, add_op2, 3, 4);

    let result = add_op1.wrapping_add(add_op2);
    ctx.set_register_value(result, dst_index);
}

pub unsafe fn str(ctx: &mut CpuContext, args: &Vec<String>) {
    let src_reg = args.get(0).unwrap();
    let reg_type = get_reg_type(src_reg);
    let src_val = ctx.get_operand_value(src_reg, reg_type);
    let dst_info = parse_reg_operand_with_offset(ctx, &args[1..].join(""));
    let dst_address = ctx.get_register_value_as::<*mut u8>(Register::from_str(&dst_info.0).unwrap());
    if let Some(ptr) = dst_address {
        let offset = dst_info.1.unwrap_or(0) as isize;
        let ptr_with_offset = ptr.offset(offset);
        if !ptr_with_offset.is_null() {
            (ptr_with_offset as *mut u64).write_unaligned(src_val);
        } else {
            panic!("Attempted to write to a null pointer");
        }
    } else {
        panic!("Attempted to write to an invalid memory address");
    }
}

pub unsafe fn ldr(ctx: &mut CpuContext, args: &Vec<String>) {
    let dst_reg = args.get(0).unwrap();
    let (_dst_reg_type, dst_index) = ctx.get_register_dst_or_src(dst_reg);
    let src_info = parse_reg_operand_with_offset(ctx, &args[1..].join(""));
    let src_value = ctx.get_register_value_as::<*mut u8>(Register::from_str(&src_info.0).unwrap());
    if let Some(src_ptr) = src_value {
        let offset = src_info.1.unwrap_or(0) as isize;
        let ptr_with_offset = src_ptr.offset(offset);
        if !ptr_with_offset.is_null() {
            let value = ptr_with_offset.read_u64();
            ctx.set_register_value(value, dst_index);
        } else {
            panic!("Attempted to read from a null pointer");
        }
    } else {
        panic!("Attempted to read from an invalid memory address");
    }
}