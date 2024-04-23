pub trait PointerExt {
    unsafe fn read_ptr(self) -> *mut u8;
    unsafe fn read_u64(self) -> u64;
    unsafe fn read_u32(self) -> u32;
    unsafe fn read_u8(self) -> u8;
}

impl PointerExt for *mut u8 {
    unsafe fn read_ptr(self) -> *mut u8 {
        assert!(!self.is_null(), "Null pointer dereference");
        assert_eq!(self.align_offset(std::mem::align_of::<*mut u8>()), 0, "Pointer must be aligned");
        *(self as *const u64) as *mut u8
    }

    unsafe fn read_u64(self) -> u64 {
        assert!(!self.is_null(), "Null pointer dereference");
        assert_eq!(self.align_offset(std::mem::align_of::<u64>()), 0, "Pointer must be aligned to 8 bytes");
        *(self as *const u64)
    }

    unsafe fn read_u32(self) -> u32 {
        assert!(!self.is_null(), "Null pointer dereference");
        assert_eq!(self.align_offset(std::mem::align_of::<u32>()), 0, "Pointer must be aligned to 4 bytes");
        *(self as *const u32)
    }

    unsafe fn read_u8(self) -> u8 {
        assert!(!self.is_null(), "Null pointer dereference");
        *(self as *const u8)
    }
}