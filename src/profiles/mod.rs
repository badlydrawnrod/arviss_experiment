//! Some implementations of hardware profiles.

pub mod basic_mem;
pub mod rv32icpu;

pub mod basic {
    pub use crate::profiles::basic_mem::*;
    pub use crate::profiles::rv32icpu::*;
}