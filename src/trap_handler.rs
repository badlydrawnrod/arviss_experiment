//! Trap handling.

use crate::Address;

#[derive(Debug, Clone, Copy)]
pub enum TrapCause {
    // Non-interrupt traps.
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction(u32),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault(Address),
    StoreAddressMisaligned,
    StoreAccessFault(Address),
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    // Interrupts.
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
}

// Trap handling is a trait.
pub trait TrapHandler {
    fn trap_cause(&self) -> Option<TrapCause>;

    fn clear_trap(&mut self);

    fn handle_trap(&mut self, cause: TrapCause);

    fn is_trapped(&self) -> bool {
        self.trap_cause().is_some()
    }

    fn handle_ecall(&mut self) {
        self.handle_trap(TrapCause::EnvironmentCallFromMMode)
    }

    fn handle_ebreak(&mut self) {
        self.handle_trap(TrapCause::Breakpoint)
    }
}
