//! Registers.

use std::fmt::Display;

/// A RISC-V register index.
#[derive(Copy, Clone, Debug)]
pub struct Reg(u32);

impl Reg {
    pub const ZERO: Reg = Reg(0);
    pub const RA: Reg = Reg(1);
    pub const SP: Reg = Reg(2);

    // GP,
    // TP,
    // T0,
    // T1,
    // T2,
    // S0,
    // S1,
    // A0,
    // A1,
    // A2,
    // A3,
    // A4,
    // A5,
    // A6,
    // A7,
    // S2,
    // S3,
    // S4,
    // S5,
    // S6,
    // S7,
    // S8,
    // S9,
    // S10,
    // S11,
    // T3,
    // T4,
    // T5,
    // T6,

    #[inline]
    pub fn new(r: u32) -> Self {
        Reg(r % 32)
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Reg::new({})", self.0))
    }
}

impl From<u32> for Reg {
    #[inline]
    fn from(r: u32) -> Self {
        Reg(r)
    }
}

impl From<Reg> for usize {
    #[inline]
    fn from(r: Reg) -> Self {
        r.0 as usize % 32
    }
}
