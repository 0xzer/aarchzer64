use crate::cpu::context::CpuContext;
use crate::instructions::base::basic::*;

pub(crate) type InstructionFn = unsafe fn(&mut CpuContext, &Vec<String>);
pub const INSTRUCTION_CALL_TABLE: &[InstructionFn] = &[
    mov,
    add,
];
