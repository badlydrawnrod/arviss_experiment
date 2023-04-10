use super::{
    cpu_types::{Address, CoreCpu, Xreg},
    memory::{BasicMem, Mem, MemoryResult},
    tobits::Reg,
    trap_handler::{TrapCause, TrapHandler},
};

#[derive(Default, Clone, Copy)]
pub struct TrapState {
    cause: Option<TrapCause>,
}

pub struct Rv32iCpu<M> {
    pc: u32,               // The program counter.
    next_pc: u32,          // The program counter for the next instruction.
    xreg: [u32; 32],       // Regular registers, x0-x31.
    mem: M,                // Memory.
    trap_state: TrapState, // The current trap state.
}

impl Default for Rv32iCpu<BasicMem> {
    fn default() -> Self {
        Self::new()
    }
}

impl Rv32iCpu<BasicMem> {
    pub fn new() -> Self {
        Self::with_mem(BasicMem::new())
    }

    pub fn with_mem(mem: BasicMem) -> Self {
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
    fn get_pc(&self) -> Address {
        self.pc
    }

    fn fetch(&mut self) -> MemoryResult<u32> {
        self.pc = self.next_pc;
        match self.mem.read32(self.pc) {
            Ok(ins) if (ins & 0b11) == 0b11 => {
                // 32-bit instruction.
                self.next_pc = self.pc.wrapping_add(4);
                Ok(ins)
            }
            Ok(ins) => {
                // 16-bit instruction.
                self.next_pc = self.pc.wrapping_add(2);
                Ok(ins & 0xffff)
            }
            Err(e) => Err(e),
        }
    }

    fn set_next_pc(&mut self, address: Address) {
        self.next_pc = address;
    }

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

    fn handle_trap(&mut self, cause: TrapCause, _value: u32) {
        self.trap_state.cause = Some(cause);
    }
}
