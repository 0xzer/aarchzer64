use crate::{cpu::{context::CpuContext, registers::Register}, instructions::common::{do_operand_bitwise_operation, get_reg_type, parse_dst_with_offset}};

pub unsafe fn mov(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let mut mov_op = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    do_operand_bitwise_operation(&args, &mut mov_op, 2, 3);
    ctx.set_register_value(mov_op, dst_index);
}

pub unsafe fn add(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let add_op1 = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let mut add_op2 = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    do_operand_bitwise_operation(&args, &mut add_op2, 3, 4);

    let result = add_op1.wrapping_add(add_op2);
    ctx.set_register_value(result, dst_index);
}

pub unsafe fn str(ctx: &mut CpuContext, args: &Vec<String>) {
    let src_reg = args.get(0).unwrap();
    let reg_type = get_reg_type(src_reg);
    let src_val = ctx.get_operand_value(src_reg, reg_type);
    let dst_info = parse_dst_with_offset(ctx, &args[1..].join(""));
    let dst_value = ctx.get_register_value(Register::from_str(&dst_info.0).unwrap());
    if let Some(offset) = dst_info.1 {
        dst_value.write_to_pointer(src_val, Some(offset))
    } else {
        dst_value.write_to_pointer(src_val, None);
    }
}