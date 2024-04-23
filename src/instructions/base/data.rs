use crate::cpu::context::CpuContext;

pub unsafe fn mov(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let mov_op = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    ctx.set_register_value(mov_op, dst_index);
}