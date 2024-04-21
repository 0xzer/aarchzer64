use crate::cpu::register_value::RegisterValue;

pub unsafe fn do_operand_bitwise_operation(
    args: &Vec<String>,
    operand: &mut RegisterValue,
    sh_operation_index: usize,
    sh_operation_index_value: usize,
) {
    if let Some(sh_operation) = args.get(sh_operation_index) {
        let sh_val = args.get(sh_operation_index_value)
            .expect("failed to retrieve bit operation value from instruction")
            .replace("#", "");
        operand.do_bitwise_operation(sh_operation, sh_val)
    };
}