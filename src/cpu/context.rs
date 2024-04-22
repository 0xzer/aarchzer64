use crate::cpu::stack::Stack;
use crate::cpu::flags::Flags;
use crate::cpu::registers::Register;
use crate::cpu::errors::cpu::CpuError;
use crate::instructions::instruction::Instructions;
use crate::instructions::instruction_table::INSTRUCTION_CALL_TABLE;

use super::register_value::TryFromU64;

#[derive(Debug, Clone, Copy)]
pub enum RegisterType {
    Reg64,
    Reg32,
}

#[derive(Debug)]
pub struct CpuContext {
    pub stack: Stack,
    pc: u64,
    instructions: Instructions,
    // x31 = SP, but lets have the SP in the Stack struct instead
    registers: [u64; 30], // General-purpose registers x0-x30
    flags: Flags
}

impl CpuContext {
    pub fn new_from_text(stack_size: usize, instructions: &str,) -> Self {
        CpuContext {
            stack: Stack::new(stack_size),
            pc: 0,
            instructions: Instructions::new_from_text(instructions),
            registers: [0; 30],
            flags: Flags {
                z: false,
                n: false,
                c: false,
                v: false,
            }
        }
    }

    pub fn get_registers(&self) -> [u64; 30] {
        self.registers
    }

    pub fn get_register_value(&self, register: Register) -> u64 {
        self.registers[register.to_index()]
    }

    pub fn get_register_value_as<T>(&self, register: Register) -> Option<T>
    where
        T: TryFromU64<T>,
    {
        let register_value = self.registers[register.to_index()];
        T::try_from_u64(register_value)
    }

    pub fn get_register_value_index(&self, index: usize) -> u64 {
        self.registers[index]
    }

    pub fn set_register_value(&mut self, value: u64, reg_index: usize) {
        self.registers[reg_index] = value
    }

    pub fn get_operand_value(&mut self, operand: &String, reg_type: RegisterType) -> u64 {
        let mut chars = operand.chars();
        match chars.next() {
            Some('s') => {
                match chars.next() {
                    // sp
                    Some('p') => unsafe { self.stack.get_sp_ptr() },
                    // preserve for simd registers later if needed
                    _ => 0,
                }
            },
            Some('x') => {
                let reg_index = self.get_register_index(&chars.as_str().replace(",", ""), 0);
                self.registers[reg_index]
            },
            Some('w') => {
                let reg_index = self.get_register_index(&chars.as_str().replace(",", ""), 0);
                let current_value = self.registers[reg_index];
                current_value as u32 as u64
            },
            _ => {
                let mut radix: u32 = 10;
                let raw_value = operand.replace("#", "").replace(",", "");
                let value = if raw_value.starts_with("0x") {
                    radix = 16;
                    &raw_value[2..]
                } else {
                    &raw_value
                };
                match reg_type {
                    RegisterType::Reg32 => u32::from_str_radix(value, radix).unwrap() as u64,
                    RegisterType::Reg64 => u64::from_str_radix(value, radix).unwrap(),
                }
            },
        }
    }

    pub fn get_register_dst_or_src(&mut self, operand: &String) -> (RegisterType, usize) {
        let mut chars = operand.chars();
        let reg_type = chars.next().unwrap();
        let reg_index = self.get_register_index(chars.as_str(), 1);
        (
            if reg_type == 'w' { RegisterType::Reg32 } else { RegisterType::Reg64 },
            reg_index
        )
    }

    pub fn get_register_index(&self, operand: &str, seperator: usize) -> usize {
        operand
        .chars()
        .take(operand.chars().count() - seperator)
        .fold(0usize, |acc, c| acc * 10 + c.to_digit(10).unwrap() as usize)
    }

    pub fn execute(&mut self) -> Result<(), CpuError> {
        let instructions_to_execute: Vec<(usize, Vec<String>)> = self.instructions.raw
            .iter()
            .map(|instruction| (instruction.instruction_mnemonic as usize, instruction.instruction_args.clone()))
            .collect();
        for (mnemonic, args) in instructions_to_execute {
            unsafe { INSTRUCTION_CALL_TABLE[mnemonic](self, &args); };
        }
        Ok(())
    }
}