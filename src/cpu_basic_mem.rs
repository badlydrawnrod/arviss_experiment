//! An RV32I CPU using the [`BasicMem`] profile.

use crate::{basic_mem::BasicMem, rv32icpu::Rv32iCpu};

impl Default for Rv32iCpu<BasicMem> {
    fn default() -> Self {
        Self::with_mem(BasicMem::new())
    }
}
