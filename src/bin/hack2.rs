use arviss_experiment::{
    decode, BasicCpu, BasicMem, BasicTrapHandler, BusCode, Disassembler, Loader, Mem, MemoryResult,
    TrapCause, TrapHandler,
};

pub fn main() {
    let mut rv32i_cpu = Rv32iCpu::<BasicMem>::new();
    rv32i_cpu.add(10, 11, 12);

    let mut rv32i_cpu_with_trap_handler = Rv32iCpu::<BasicMem, BasicTrapHandler>::new();
    rv32i_cpu_with_trap_handler.add(10, 11, 12);

    let mut rv32f_cpu = Rv32fCpu {
        pc: 0,
        xreg: Default::default(),
        freg: Default::default(),
    };
    rv32f_cpu.add(10, 11, 12);
    rv32f_cpu.fadd_s(10, 11, 12, 0);
}

pub type Address = u32;

trait CoreCpu {
    fn update_pc(&mut self);
    fn read8(&self, address: Address) -> MemoryResult<u8>;
}

trait Xreg {
    fn rx(&self, reg: usize) -> u32;
    fn wx(&mut self, reg: usize, val: u32);
}

trait Freg {
    fn rf(&self, reg: usize) -> f32;
    fn wf(&mut self, reg: usize, val: f32);
}

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

struct Rv32fCpu {
    pc: u32,         // The program counter.
    xreg: [u32; 32], // Regular registers, x0-x31.
    freg: [f32; 32], // Floating point registers, f0-f31.
}

impl<M, T> CoreCpu for Rv32iCpu<M, T>
where
    M: Mem,
    // T: TrapHandler<Item = ()>,
{
    fn update_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
    }

    fn read8(&self, address: Address) -> MemoryResult<u8> {
        self.mem.read8(address)
    }
}

impl<M, T> Xreg for Rv32iCpu<M, T>
where
    M: Mem,
    // T: TrapHandler<Item = ()>,
{
    fn rx(&self, reg: usize) -> u32 {
        self.xreg[reg]
    }

    fn wx(&mut self, reg: usize, val: u32) {
        self.xreg[reg] = val;
    }
}

impl CoreCpu for Rv32fCpu {
    fn update_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
    }

    fn read8(&self, address: Address) -> MemoryResult<u8> {
        todo!()
    }
}

impl Xreg for Rv32fCpu {
    fn rx(&self, reg: usize) -> u32 {
        self.xreg[reg]
    }

    fn wx(&mut self, reg: usize, val: u32) {
        self.xreg[reg] = val;
    }
}

impl Freg for Rv32fCpu {
    fn rf(&self, reg: usize) -> f32 {
        self.freg[reg]
    }

    fn wf(&mut self, reg: usize, val: f32) {
        self.freg[reg] = val;
    }
}

trait Rv32i: CoreCpu + Xreg {
    // Arithmetic instructions.
    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 + rs2, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_add(self.rx(rs2)));
        self.update_pc();
        self.wx(0, 0);
    }

    fn lb(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m8(rs1 + imm_i)), pc += 4
        match self.read8(self.rx(rs1).wrapping_add(iimm)) {
            Ok(byte) => {
                self.wx(rd, (((byte as i8) as i16) as i32) as u32); // TODO: this should be a function.
                self.update_pc();
                self.wx(0, 0);
            }
            Err(_) => {
                // self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }
}

trait Rv32f: Rv32i + Freg {
    // Arithmetic instructions.
    fn fadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize) {
        // rd <- rs1 + rs2
        self.wf(rd, self.rf(rs1) + self.rf(rs2));
        self.update_pc();
    }
}

impl<M, T> Rv32i for Rv32iCpu<M, T> where M: Mem {}

impl Rv32i for Rv32fCpu {}
impl Rv32f for Rv32fCpu {}