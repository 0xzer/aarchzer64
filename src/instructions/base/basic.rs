use crate::{cpu::context::CpuContext, instructions::common::do_operand_bitwise_operation};

pub unsafe fn mov(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst(args.get(0).unwrap());
    let mut mov_op = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    do_operand_bitwise_operation(&args, &mut mov_op, 2, 3);

    ctx.set_register_value(mov_op, dst_index);
}

pub unsafe fn add(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst(args.get(0).unwrap());
    let add_op1 = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let mut add_op2 = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    do_operand_bitwise_operation(&args, &mut add_op2, 3, 4);

    let result = add_op1.wrapping_add(add_op2);
    ctx.set_register_value(result, dst_index);
}