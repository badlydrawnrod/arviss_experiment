//! Implementations of CPU and memory back ends.

mod basic_mem;
mod rv32icpu;

/// Memory back ends.
pub mod memory {
    use super::basic_mem;
    /// A back end for the "basic" memory layout.
    pub mod basic {
        pub use super::basic_mem::*;
    }
}

/// CPU back ends.
pub mod cpus {
    use super::rv32icpu;
    /// A back end for an integer only RV32I CPU.
    pub mod rv32i {
        pub use super::rv32icpu::*;
    }
}
