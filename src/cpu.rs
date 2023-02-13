pub struct Cpu {
    pc: u32,         // The program counter.
    xreg: [u32; 32], // Regular registerx, x0-x31.
    freg: [f32; 32], // Floating point registers, f0-f31.
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            xreg: Default::default(),
            freg: Default::default(),
        }
    }
}
