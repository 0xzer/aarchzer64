
#[derive(Debug, Clone, Copy)]
pub enum RegisterValue {
    Value32(u32),
    Value64(u64),
    Ptr(*mut u8),
}

pub enum BitwiseOperation {
    Unknown,
    LSL,
    LSR,
    ROR,
}

impl BitwiseOperation {
    pub fn from_str(operation: &str) -> Self {
        match operation {
            "LSL" => Self::LSL,
            "LSR" => Self::LSR,
            "ROR" => Self::ROR,
            _ => Self::Unknown,
        }
    }
}

impl RegisterValue {
    pub fn to_usize(self) -> usize {
        match self {
            RegisterValue::Value32(v) => v as usize,
            RegisterValue::Value64(v) => v as usize,
            RegisterValue::Ptr(v) => v as usize,
        }
    }

    pub fn get_lower32_bits(self) -> RegisterValue {
        match self {
            RegisterValue::Value32(v) => RegisterValue::Value32(v),
            RegisterValue::Value64(v) => RegisterValue::Value32(v as u32),
            RegisterValue::Ptr(v) => RegisterValue::Value32(v as usize as u32),
        }
    }

    pub unsafe fn write_to_pointer(self, value: RegisterValue, offset: Option<usize>) {
        match self {
            RegisterValue::Ptr(base_pointer) => {
                assert!(!base_pointer.is_null(), "Attempted to write value {:?} to null ptr", value);
                let pointer = if let Some(offset) = offset {
                    base_pointer.wrapping_add(offset)
                } else {
                    base_pointer
                };
                match value {
                    RegisterValue::Value32(val) => {
                        assert_eq!(pointer.align_offset(std::mem::align_of::<u32>()), 0, "Pointer must be aligned to 4 bytes for u32");
                        *(pointer as *mut u32) = val;
                    },
                    RegisterValue::Value64(val) => {
                        assert_eq!(pointer.align_offset(std::mem::align_of::<u64>()), 0, "Pointer must be aligned to 8 bytes for u64");
                        *(pointer as *mut u64) = val;
                    },
                    RegisterValue::Ptr(val) => {
                        assert_eq!(pointer.align_offset(std::mem::align_of::<*mut u8>()), 0, "Pointer must be aligned for *mut u8");
                        *(pointer as *mut *mut u8) = val;
                    },
                }
            },
            _ => panic!("attempted to write value {:?} to non-pointer RegisterValue", value)
        }
    }

    pub fn wrapping_add(self, op: RegisterValue) -> RegisterValue {
        match (self, op) {
            (RegisterValue::Value32(v1), RegisterValue::Value32(v2)) => RegisterValue::Value32(v1.wrapping_add(v2)),
            (RegisterValue::Value64(v1), RegisterValue::Value64(v2)) => RegisterValue::Value64(v1.wrapping_add(v2)),
            (RegisterValue::Ptr(v1), RegisterValue::Value64(v2)) => RegisterValue::Ptr(v1.wrapping_add(v2 as usize)),
            (RegisterValue::Value32(_), RegisterValue::Value64(_)) => panic!("32-bit register does not allow addition with 64-bit register"),
            (RegisterValue::Value32(v1), RegisterValue::Ptr(v2)) => RegisterValue::Ptr(v2.wrapping_add(v1 as usize)),
            (RegisterValue::Value64(_), RegisterValue::Value32(_)) => panic!("64-bit register does not allow addition with 32-bit register"),
            (RegisterValue::Value64(v1), RegisterValue::Ptr(v2)) => RegisterValue::Ptr(v2.wrapping_add(v1 as usize)),
            (RegisterValue::Ptr(v1), RegisterValue::Value32(v2)) => RegisterValue::Ptr(v1.wrapping_add(v2 as usize)),
            (RegisterValue::Ptr(v1), RegisterValue::Ptr(v2)) => RegisterValue::Ptr(v1.wrapping_add(v2 as u64 as usize)),
        }
    }

    pub fn do_bitwise_operation(&mut self, operation: &str, sh: String) {
        let v = sh.parse::<u32>().expect("failed to parse shift value to u32");
        match BitwiseOperation::from_str(operation) {
            BitwiseOperation::LSL => self.do_shl(v),
            BitwiseOperation::LSR => self.do_shr(v),
            BitwiseOperation::ROR => self.do_ror(v),
            _ => {},
        }
    }

    fn do_shl(&mut self, v: u32) {
        *self = match *self {
            RegisterValue::Value32(value) => RegisterValue::Value32(value.wrapping_shl(v)),
            RegisterValue::Value64(value) => RegisterValue::Value64(value.wrapping_shl(v)),
            RegisterValue::Ptr(_) => panic!("Logical left shift operation not supported on pointers"),
        };
    }

    fn do_ror(&mut self, v: u32) {
        *self = match *self {
            RegisterValue::Value32(value) => RegisterValue::Value32(value.rotate_right(v)),
            RegisterValue::Value64(value) => RegisterValue::Value64(value.rotate_right(v)),
            RegisterValue::Ptr(_) => panic!("Rotate right operation not supported on pointers"),
        };
    }

    fn do_shr(&mut self, v: u32) {
        *self = match *self {
            RegisterValue::Value32(value) => RegisterValue::Value32(value.wrapping_shr(v)),
            RegisterValue::Value64(value) => RegisterValue::Value64(value.wrapping_shr(v)),
            RegisterValue::Ptr(_) => panic!("Logical right shift operation not supported on pointers"),
        };
    }
}

pub trait TryFromRegisterValue<T> {
    fn try_from_register_value(value: RegisterValue) -> Option<T>;
}

impl TryFromRegisterValue<u32> for u32 {
    fn try_from_register_value(value: RegisterValue) -> Option<u32> {
        if let RegisterValue::Value32(v) = value {
            Some(v)
        } else {
            None
        }
    }
}

impl TryFromRegisterValue<u64> for u64 {
    fn try_from_register_value(value: RegisterValue) -> Option<u64> {
        if let RegisterValue::Value64(v) = value {
            Some(v)
        } else {
            None
        }
    }
}

impl TryFromRegisterValue<*mut u8> for *mut u8 {
    fn try_from_register_value(value: RegisterValue) -> Option<*mut u8> {
        if let RegisterValue::Ptr(p) = value {
            Some(p)
        } else {
            None
        }
    }
}
