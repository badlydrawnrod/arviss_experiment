use super::{
    cpu_types::{Address, CoreCpu, Xreg},
    memory::{BasicMem, Mem},
    tobits::Reg,
    trap_handler::{BasicTrapHandler, TrapCause, TrapHandler},
};

pub struct DummyTrapHandler;

impl TrapHandler for std::marker::PhantomData<DummyTrapHandler> {
    fn handle_trap(&self, _cause: TrapCause) {}

    fn handle_ecall(&self) {}

    fn handle_ebreak(&self) {}
}

pub struct Rv32iCpu<M, T = std::marker::PhantomData<DummyTrapHandler>> {
    pc: u32,         // The program counter.
    next_pc: u32,    // The program counter for the next instruction.
    xreg: [u32; 32], // Regular registers, x0-x31.
    mem: M,          // Memory.
    trap_handler: T, // Trap handler.
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
            mem: mem,
            trap_handler: std::marker::PhantomData::<DummyTrapHandler>,
        }
    }
}

impl Rv32iCpu<BasicMem, BasicTrapHandler> {
    pub fn new() -> Self {
        Self::with_mem(BasicMem::new())
    }

    pub fn with_mem(mem: BasicMem) -> Self {
        Self {
            pc: 0,
            next_pc: 0,
            xreg: Default::default(),
            mem: mem,
            trap_handler: BasicTrapHandler::new(),
        }
    }
}

impl<M, T> CoreCpu for Rv32iCpu<M, T>
where
    M: Mem,
    T: TrapHandler,
{
    fn get_pc(&self) -> Address {
        self.pc
    }

    fn cycle_next_pc(&mut self) -> Address {
        self.pc = self.next_pc;
        // TODO: this may not always be the case, e.g., for RV32C. Let's cross that bridge when we come to it.
        self.next_pc = self.next_pc.wrapping_add(4);
        self.pc
    }

    fn set_next_pc(&mut self, address: Address) {
        self.next_pc = address;
    }

    fn read8(&self, address: Address) -> super::memory::MemoryResult<u8> {
        self.mem.read8(address)
    }

    fn read16(&self, address: Address) -> super::memory::MemoryResult<u16> {
        self.mem.read16(address)
    }

    fn read32(&self, address: Address) -> super::memory::MemoryResult<u32> {
        self.mem.read32(address)
    }

    fn write8(&mut self, address: Address, value: u8) -> super::memory::MemoryResult<()> {
        self.mem.write8(address, value)
    }

    fn write16(&mut self, address: Address, value: u16) -> super::memory::MemoryResult<()> {
        self.mem.write16(address, value)
    }

    fn write32(&mut self, address: Address, value: u32) -> super::memory::MemoryResult<()> {
        self.mem.write32(address, value)
    }

    fn handle_trap(&mut self, cause: TrapCause) {
        self.trap_handler.handle_trap(cause)
    }
}

impl<M, T> Xreg for Rv32iCpu<M, T>
where
    M: Mem,
    T: TrapHandler,
{
    fn rx(&self, reg: Reg) -> u32 {
        self.xreg[reg as usize]
    }

    fn wx(&mut self, reg: Reg, val: u32) {
        self.xreg[reg as usize] = val;
        self.xreg[0] = 0;
    }
}
