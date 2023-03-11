use super::{
    cpu_types::{Rv32i, Xreg},
    memory::{BasicMem, Mem},
    trap_handler::{BasicTrapHandler, TrapCause, TrapHandler},
};

struct DummyTrapHandler;

struct Rv32iCpu<M, T = std::marker::PhantomData<DummyTrapHandler>> {
    pc: u32,         // The program counter.
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
            xreg: Default::default(),
            mem: mem,
            trap_handler: BasicTrapHandler::new(),
        }
    }

    #[inline]
    fn handle_trap(&mut self, cause: TrapCause) {
        self.trap_handler.handle_trap(cause)
    }
}

// impl<T, U> Xreg for Rv32iCpu<T, U>
// where
//     T: Mem,
//     U: TrapHandler<Item = ()>,
// {
//     fn rx(&self, reg: usize) -> u32 {
//         self.xreg[reg]
//     }

//     fn wx(&mut self, reg: usize, val: u32) {
//         self.xreg[reg] = val
//     }
// }

