use crate::cpu::context::CpuContext;

pub unsafe fn lsr(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let lsr_op = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let lsr_val = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    ctx.set_register_value(lsr_op.wrapping_shr(lsr_val as u32), dst_index);
}

pub unsafe fn lsl(ctx: &mut CpuContext, args: &Vec<String>) {
    let (reg_type, dst_index) = ctx.get_register_dst_or_src(args.get(0).unwrap());
    let lsl_op = ctx.get_operand_value(args.get(1).unwrap(), reg_type);
    let lsl_val = ctx.get_operand_value(args.get(2).unwrap(), reg_type);
    ctx.set_register_value(lsl_op.wrapping_shr(lsl_val as u32), dst_index);
}