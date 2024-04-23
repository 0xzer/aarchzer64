use crate::cpu::context::CpuContext;
use crate::instructions::common::do_operand_bitwise_operation;

pub unsafe fn add(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let add_op1 = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let mut add_op2 = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    add_op2 = do_operand_bitwise_operation(&args, add_op2, 3, 4);

    let result = add_op1.wrapping_add(add_op2);
    ctx.set_register_value(result, dst_index);
}

pub unsafe fn sub(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let sub_op1 = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let mut sub_op2 = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    sub_op2 = do_operand_bitwise_operation(&args, sub_op2, 3, 4);

    let result = sub_op1.wrapping_sub(sub_op2);
    ctx.set_register_value(result, dst_index);
}