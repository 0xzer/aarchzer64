#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30,
    W0, W1, W2, W3, W4, W5, W6, W7,
    W8, W9, W10, W11, W12, W13, W14, W15,
    W16, W17, W18, W19, W20, W21, W22, W23,
    W24, W25, W26, W27, W28, W29, W30,
}

impl Register {
    pub fn from_str(reg: &str) -> Option<Self> {
        match reg {
            "x0" => Some(Self::X0), "x1" => Some(Self::X1), "x2" => Some(Self::X2), "x3" => Some(Self::X3),
            "x4" => Some(Self::X4), "x5" => Some(Self::X5), "x6" => Some(Self::X6), "x7" => Some(Self::X7),
            "x8" => Some(Self::X8), "x9" => Some(Self::X9), "x10" => Some(Self::X10), "x11" => Some(Self::X11),
            "x12" => Some(Self::X12), "x13" => Some(Self::X13), "x14" => Some(Self::X14), "x15" => Some(Self::X15),
            "x16" => Some(Self::X16), "x17" => Some(Self::X17), "x18" => Some(Self::X18), "x19" => Some(Self::X19),
            "x20" => Some(Self::X20), "x21" => Some(Self::X21), "x22" => Some(Self::X22), "x23" => Some(Self::X23),
            "x24" => Some(Self::X24), "x25" => Some(Self::X25), "x26" => Some(Self::X26), "x27" => Some(Self::X27),
            "x28" => Some(Self::X28), "x29" => Some(Self::X29), "x30" => Some(Self::X30),
            "w0" => Some(Self::W0), "w1" => Some(Self::W1), "w2" => Some(Self::W2), "w3" => Some(Self::W3),
            "w4" => Some(Self::W4), "w5" => Some(Self::W5), "w6" => Some(Self::W6), "w7" => Some(Self::W7),
            "w8" => Some(Self::W8), "w9" => Some(Self::W9), "w10" => Some(Self::W10), "w11" => Some(Self::W11),
            "w12" => Some(Self::W12), "w13" => Some(Self::W13), "w14" => Some(Self::W14), "w15" => Some(Self::W15),
            "w16" => Some(Self::W16), "w17" => Some(Self::W17), "w18" => Some(Self::W18), "w19" => Some(Self::W19),
            "w20" => Some(Self::W20), "w21" => Some(Self::W21), "w22" => Some(Self::W22), "w23" => Some(Self::W23),
            "w24" => Some(Self::W24), "w25" => Some(Self::W25), "w26" => Some(Self::W26), "w27" => Some(Self::W27),
            "w28" => Some(Self::W28), "w29" => Some(Self::W29), "w30" => Some(Self::W30),
            _ => None,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Register::X0 | Register::W0 => 0,
            Register::X1 | Register::W1 => 1,
            Register::X2 | Register::W2 => 2,
            Register::X3 | Register::W3 => 3,
            Register::X4 | Register::W4 => 4,
            Register::X5 | Register::W5 => 5,
            Register::X6 | Register::W6 => 6,
            Register::X7 | Register::W7 => 7,
            Register::X8 | Register::W8 => 8,
            Register::X9 | Register::W9 => 9,
            Register::X10 | Register::W10 => 10,
            Register::X11 | Register::W11 => 11,
            Register::X12 | Register::W12 => 12,
            Register::X13 | Register::W13 => 13,
            Register::X14 | Register::W14 => 14,
            Register::X15 | Register::W15 => 15,
            Register::X16 | Register::W16 => 16,
            Register::X17 | Register::W17 => 17,
            Register::X18 | Register::W18 => 18,
            Register::X19 | Register::W19 => 19,
            Register::X20 | Register::W20 => 20,
            Register::X21 | Register::W21 => 21,
            Register::X22 | Register::W22 => 22,
            Register::X23 | Register::W23 => 23,
            Register::X24 | Register::W24 => 24,
            Register::X25 | Register::W25 => 25,
            Register::X26 | Register::W26 => 26,
            Register::X27 | Register::W27 => 27,
            Register::X28 | Register::W28 => 28,
            Register::X29 | Register::W29 => 29,
            Register::X30 | Register::W30 => 30,
        }
    }
}
