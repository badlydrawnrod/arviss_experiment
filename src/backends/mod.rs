//! Some implementations of hardware profiles.

mod basic_mem;
mod rv32icpu;

/// Memory back ends.
pub mod memory {
    /// A back end for the "basic" memory layout.
    pub mod basic {
        pub use super::super::basic_mem::*;
    }
}

/// CPU back ends.
pub mod cpus {
    /// A back end for an integer only RV32I CPU.
    pub mod rv32i {
        pub use super::super::rv32icpu::*;
    }
}
