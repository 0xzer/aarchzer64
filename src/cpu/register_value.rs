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
    pub fn wrapping_add(self, op: RegisterValue) -> RegisterValue {
        match (self, op) {
            (RegisterValue::Value32(v1), RegisterValue::Value32(v2)) => RegisterValue::Value32(v1.wrapping_add(v2)),
            (RegisterValue::Value64(v1), RegisterValue::Value64(v2)) => RegisterValue::Value64(v1.wrapping_add(v2)),
            (RegisterValue::Ptr(v1), RegisterValue::Value64(v2)) => RegisterValue::Ptr(v1.wrapping_add(v2 as usize)),
            (RegisterValue::Value32(_), RegisterValue::Value64(_)) => panic!("x32 register does not allow addition with x64 register"),
            (RegisterValue::Value32(v1), RegisterValue::Ptr(v2)) => RegisterValue::Ptr(v2.wrapping_add(v1 as usize)),
            (RegisterValue::Value64(_), RegisterValue::Value32(_)) => panic!("x64 register does not allow addition with x32 register"),
            (RegisterValue::Value64(v1), RegisterValue::Ptr(v2)) => RegisterValue::Ptr(v2.wrapping_add(v1 as usize)),
            (RegisterValue::Ptr(v1), RegisterValue::Value32(v2)) => RegisterValue::Ptr(v1.wrapping_add(v2 as usize)),
            (RegisterValue::Ptr(v1), RegisterValue::Ptr(v2)) => RegisterValue::Ptr(v1.wrapping_add((v2 as u64) as usize)),
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
