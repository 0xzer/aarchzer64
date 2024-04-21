
#[derive(Debug)]
pub(crate) struct Flags {
    pub z: bool, // Zero flag
    pub n: bool, // Negative flag
    pub c: bool, // Carry flag
    pub v: bool, // Overflow flag
}