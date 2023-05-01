//! An RV32I CPU with integer registers but not floating point.

use crate::{
    memory::{Address, Loader, Mem, MemoryResult},
    reg::Reg,
};

pub use crate::cpu::{CoreCpu, Xreg};
pub use crate::trap::{TrapCause, TrapHandler};

/// The current trap state of the CPU.
#[derive(Default, Clone, Copy)]
pub struct TrapState {
    cause: Option<TrapCause>,
}

/// A basic RV32I CPU with integer registers but no floating point.
#[derive(Default)]
pub struct Rv32iCpu<M>
where
    M: Mem,
{
    pc: u32,               // The program counter.
    next_pc: u32,          // The program counter for the next instruction.
    xreg: [u32; 32],       // Regular registers, x0-x31.
    mem: M,                // Memory.
    trap_state: TrapState, // The current trap state.
}

impl<M> Rv32iCpu<M>
where
    M: Mem,
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

impl<M> CoreCpu for Rv32iCpu<M>
where
    M: Mem,
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

impl<M> Mem for Rv32iCpu<M>
where
    M: Mem,
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

impl<M> Loader for Rv32iCpu<M>
where
    M: Mem + Loader,
{
    fn write_bytes(&mut self, start: crate::Address, bytes: &[u8]) -> crate::MemoryResult<()> {
        self.mem.write_bytes(start, bytes)
    }
}

impl<M> Xreg for Rv32iCpu<M>
where
    M: Mem,
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

impl<M> TrapHandler for Rv32iCpu<M>
where
    M: Mem,
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
