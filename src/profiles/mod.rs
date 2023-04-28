//! Some implementations of hardware profiles.

mod basic_mem;
mod rv32icpu;

/// CPU implementations.
pub mod cpu {
    pub use super::rv32icpu::*;
}

/// Memory implementations.
pub mod memory {
    pub use super::basic_mem::*;
}
