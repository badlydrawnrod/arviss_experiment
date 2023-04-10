#[derive(Debug, Clone, Copy)]
#[repr(u32)]
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
    fn trap_cause(&self) -> Option<TrapCause>;

    fn clear_trap(&mut self);

    fn handle_trap(&mut self, cause: TrapCause, value: u32);

    fn is_trapped(&self) -> bool {
        self.trap_cause().is_some()
    }

    fn handle_ecall(&mut self) {
        self.handle_trap(TrapCause::EnvironmentCallFromMMode, 0)
    }

    fn handle_ebreak(&mut self) {
        self.handle_trap(TrapCause::Breakpoint, 0)
    }
}
