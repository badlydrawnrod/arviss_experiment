#[derive(Debug)]
pub enum TrapCause {
    // Non-interrupt traps.
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
    // Interrupts (top bit is set).
    SupervisorSoftwareInterrupt = 0x80000001,
    MachineSoftwareInterrupt = 0x80000003,
    SupervisorTimerInterrupt = 0x80000005,
    MachineTimerInterrupt = 0x80000007,
    SupervisorExternalInterrupt = 0x80000009,
    MachineExternalInterrupt = 0x8000000b,
}

// Trap handling is a trait.
pub trait TrapHandler {
    fn handle_trap(&self, cause: TrapCause);
    fn handle_ecall(&self);
    fn handle_ebreak(&self);
}

pub struct BasicTrapHandler {
    // TODO...
}

impl BasicTrapHandler {
    pub fn new() -> Self {
        BasicTrapHandler {}
    }
}

impl TrapHandler for BasicTrapHandler {
    fn handle_trap(&self, cause: TrapCause) {
        println!("TRAP: {:#?}", cause);
    }

    fn handle_ecall(&self) {
        println!("ECALL!");
    }

    fn handle_ebreak(&self) {
        println!("EBREAK!");
    }
}
