//! Core traits used to implement RISC-V CPUs.

use crate::{
    memory::{Address, MemoryResult},
    reg::Reg,
};

/// Fetches the next instruction from memory and updates the program counter.
///
/// Provides an abstraction over the basic CPU operations that involve fetching the next instruction from memory and
/// updating the program counter.
pub trait Fetch {
    /// Returns the current value of the program counter.
    fn pc(&self) -> Address;

    /// Sets the program counter to `next_pc` and returns it.
    fn transfer(&mut self) -> Address;

    /// Sets the program counter to `next_pc`, retrieves the instruction at the memory address in the program counter,
    /// then sets `next_pc` to the address of the next instruction by adding 4 bytes for 32-bit instructions and 2 bytes
    /// for 16-bit instructions.
    fn fetch(&mut self) -> MemoryResult<u32> {
        let pc = self.transfer();
        match self.fetch32(pc) {
            Ok(ins) if (ins & 0b11) == 0b11 => {
                // 32-bit instruction.
                self.set_next_pc(pc.wrapping_add(4));
                Ok(ins)
            }
            Ok(ins) => {
                // 16-bit compressed instruction.
                self.set_next_pc(pc.wrapping_add(2));
                Ok(ins & 0xffff)
            }
            Err(e) => Err(e),
        }
    }

    /// Sets the value of `next_pc`, the address that's copied into the program counter when `fetch` is called.
    fn set_next_pc(&mut self, address: Address);

    /// Fetches a 32-bit word from memory.
    fn fetch32(&self, address: Address) -> MemoryResult<u32>;
}

/// Provides access to the base RV32I integer registers.
pub trait XRegisters {
    /// Returns the value in the given integer register.
    fn rx(&self, reg: Reg) -> u32;

    /// Writes a value to the given integer register.
    fn wx(&mut self, reg: Reg, val: u32);
}

/// Provides access to the single-precision floating point registers used by the 'F' extension.
pub trait FRegisters {
    /// Returns the value in the given floating point register.
    fn rf(&self, reg: Reg) -> f32;

    /// Writes a value to the given floating point register.
    fn wf(&mut self, reg: Reg, val: f32);
}
