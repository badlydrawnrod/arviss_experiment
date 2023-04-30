//! Hardware platforms.

/// A platform that uses an RV32ICPU with basic memory.
pub mod basic {
    pub use crate::profiles::basic_mem::*;
    pub use crate::profiles::rv32icpu::*;

    pub type BasicCpu = Rv32iCpu<BasicMem>;
}
