#[derive(Debug)]
pub(crate) struct Stack {
    mem: Vec<u8>,
    sp: usize,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Stack {
            mem: vec![0; size],
            sp: 0,
        }
    }

    pub fn get_sp_ptr(&mut self) -> *mut u8 {
        if self.sp >= self.mem.len() {
            panic!("Stack pointer out of bounds: {} (allocated size={})", self.sp, self.mem.len());
        }
        self.mem[self.sp] as *mut u8
    }
}