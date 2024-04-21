use crate::cpu::stack::Stack;
use crate::cpu::flags::Flags;
use crate::cpu::register_value::{RegisterValue, TryFromRegisterValue};
use crate::cpu::registers::Register;
use crate::cpu::errors::cpu::CpuError;
use crate::instructions::instruction::Instructions;
use crate::instructions::instruction_table::INSTRUCTION_CALL_TABLE;

#[derive(Debug, Clone, Copy)]
pub enum RegisterType {
    Reg64,
    Reg32,
}

const REGISTERS_START_STATE: RegisterValue = RegisterValue::Value64(0);

#[derive(Debug)]
pub struct CpuContext {
    pub stack: Stack,
    pc: u64,
    instructions: Instructions,
    // x31 = SP, but lets have the SP in the Stack struct instead
    registers: [RegisterValue; 30], // General-purpose registers x0-x30
    flags: Flags
}

impl CpuContext {
    pub fn new_from_text(stack_size: usize, instructions: &str,) -> Self {
        CpuContext {
            stack: Stack::new(stack_size),
            pc: 0,
            instructions: Instructions::new_from_text(instructions),
            registers: [REGISTERS_START_STATE; 30],
            flags: Flags {
                z: false,
                n: false,
                c: false,
                v: false,
            }
        }
    }

    pub fn get_registers(&self) -> [RegisterValue; 30] {
        self.registers
    }

    pub fn get_register_value(&self, register: Register) -> RegisterValue {
        self.registers[register.to_index()]
    }

    pub fn get_register_value_index(&self, index: usize) -> RegisterValue {
        self.registers[index]
    }

    pub fn get_register_value_as<T>(&self, register: Register) -> Option<T>
    where
        T: TryFromRegisterValue<T>,
    {
        T::try_from_register_value(self.registers[register.to_index()])
    }

    pub fn set_register_value(&mut self, value: RegisterValue, reg_index: usize) {
        self.registers[reg_index] = value
    }

    pub fn get_operand_value(&mut self, operand: &String, reg_type: RegisterType) -> RegisterValue {
        let mut chars = operand.chars();
        match chars.next() {
            Some('s') => {
                match chars.next() {
                    // sp
                    Some('p') => unsafe { RegisterValue::Ptr(self.stack.get_sp_ptr()) },
                    // preserve for simd registers later if needed
                    _ => RegisterValue::Value64(0),
                }
            },
            Some('x') => {
                let reg_index = self.get_register_index(&chars.as_str().replace(",", ""), 0);
                self.registers[reg_index]
            },
            Some('w') => {
                let reg_index = self.get_register_index(&chars.as_str().replace(",", ""), 0);
                let current_value = self.registers[reg_index];
                current_value.get_lower32_bits()
            },
            _ => {
                let raw_value = operand.replace("#", "").replace(",", "");
                match reg_type {
                    RegisterType::Reg32 => RegisterValue::Value32(raw_value.parse::<u32>().unwrap()),
                    RegisterType::Reg64 => RegisterValue::Value64(raw_value.parse::<u64>().unwrap()),
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