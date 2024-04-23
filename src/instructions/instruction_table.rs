use crate::cpu::context::CpuContext;
use crate::instructions::base::arithmetic::*;
use crate::instructions::base::data::*;
use crate::instructions::base::memory::*;
use crate::instructions::base::bitwise::*;

pub(crate) type InstructionFn = unsafe fn(&mut CpuContext, &Vec<String>);
pub const INSTRUCTION_CALL_TABLE: &[InstructionFn] = &[
    mov,
    add,
    str,
    ldr,
    sub,
    lsr,
    lsl,
    ldrb,
    strb,
];
