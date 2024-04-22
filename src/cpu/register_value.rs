pub trait TryFromU64<T> {
    fn try_from_u64(value: u64) -> Option<T>;
}

impl TryFromU64<u32> for u32 {
    fn try_from_u64(value: u64) -> Option<u32> {
        if value <= u32::MAX as u64 {
            Some(value as u32)
        } else {
            None
        }
    }
}

impl TryFromU64<u64> for u64 {
    fn try_from_u64(value: u64) -> Option<u64> {
        Some(value) // Directly return the value
    }
}

impl TryFromU64<*mut u8> for *mut u8 {
    fn try_from_u64(value: u64) -> Option<*mut u8> {
        Some(value as *mut u8)
    }
}
