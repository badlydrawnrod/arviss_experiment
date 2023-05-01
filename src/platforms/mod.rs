//! Hardware platforms.

/// A platform that uses an RV32ICPU with basic memory.
pub mod basic {
    pub use crate::backends::memory::basic::*;
    pub use crate::backends::cpus::rv32i::*;

    pub type BasicCpu = Rv32iCpu<BasicMem>;
}
