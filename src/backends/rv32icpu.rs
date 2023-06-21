//! An RV32I CPU with integer registers but not floating point.

use std::fmt::Display;

use crate::{
    memory::{Address, Load, Memory, MemoryResult},
    reg::Reg,
};

pub use crate::cpu::{Fetch, XRegisters};
pub use crate::trap::{Trap, TrapCause};

/// The current trap state of the CPU.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TrapState {
    cause: Option<TrapCause>,
}

/// A basic RV32I CPU with integer registers but no floating point.
#[derive(Default, PartialEq, Eq)]
pub struct Rv32iCpu<M>
where
    M: Memory,
{
    pc: u32,               // The program counter.
    next_pc: u32,          // The program counter for the next instruction.
    xreg: [u32; 32],       // Regular registers, x0-x31.
    mem: M,                // Memory.
    trap_state: TrapState, // The current trap state.
}

impl<M> Display for Rv32iCpu<M>
where
    M: Memory,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "pc: {:08x} next_pc {:08x}\n",
            self.pc, self.next_pc
        ))?;
        f.write_fmt(format_args!("xregs: {:?}\n", self.xreg))
    }
}

impl<M> Rv32iCpu<M>
where
    M: Memory,
{
    /// Createa a new CPU with default memory.
    pub fn new() -> Self
    where
        M: Default,
    {
        Self::with_mem(Default::default())
    }

    /// Creates a new CPU with caller-supplied memory.
    pub fn with_mem(mem: M) -> Self {
        Self {
            pc: 0,
            next_pc: 0,
            xreg: Default::default(),
            mem,
            trap_state: Default::default(),
        }
    }
}

impl<M> Fetch for Rv32iCpu<M>
where
    M: Memory,
{
    fn pc(&self) -> Address {
        self.pc
    }

    fn transfer(&mut self) -> Address {
        self.pc = self.next_pc;
        self.pc
    }

    fn set_next_pc(&mut self, address: Address) {
        self.next_pc = address;
    }

    fn fetch32(&self, address: Address) -> MemoryResult<u32> {
        self.read32(address)
    }
}

impl<M> Memory for Rv32iCpu<M>
where
    M: Memory,
{
    fn read8(&self, address: Address) -> MemoryResult<u8> {
        self.mem.read8(address)
    }

    fn read16(&self, address: Address) -> MemoryResult<u16> {
        self.mem.read16(address)
    }

    fn read32(&self, address: Address) -> MemoryResult<u32> {
        self.mem.read32(address)
    }

    fn write8(&mut self, address: Address, value: u8) -> MemoryResult<()> {
        self.mem.write8(address, value)
    }

    fn write16(&mut self, address: Address, value: u16) -> MemoryResult<()> {
        self.mem.write16(address, value)
    }

    fn write32(&mut self, address: Address, value: u32) -> MemoryResult<()> {
        self.mem.write32(address, value)
    }
}

impl<M> Load for Rv32iCpu<M>
where
    M: Memory + Load,
{
    fn write_bytes(&mut self, start: crate::Address, bytes: &[u8]) -> crate::MemoryResult<()> {
        self.mem.write_bytes(start, bytes)
    }
}

impl<M> XRegisters for Rv32iCpu<M>
where
    M: Memory,
{
    fn rx(&self, reg: Reg) -> u32 {
        let index: usize = Into::into(reg);
        self.xreg[index]
    }

    fn wx(&mut self, reg: Reg, val: u32) {
        let index: usize = Into::into(reg);
        self.xreg[index] = val;
        self.xreg[0] = 0;
    }
}

impl<M> Trap for Rv32iCpu<M>
where
    M: Memory,
{
    fn trap_cause(&self) -> Option<TrapCause> {
        self.trap_state.cause
    }

    fn clear_trap(&mut self) {
        self.trap_state.cause = None
    }

    fn handle_trap(&mut self, cause: TrapCause) {
        self.trap_state.cause = Some(cause);
    }
}
