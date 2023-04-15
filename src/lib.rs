//! Arviss is ***A*** ***R***ISC-***V*** ***I***nstruction ***S***et ***S***imulator for 32-bit variants of the
//! [RISC-V](https://en.wikipedia.org/wiki/RISC-V) instruction set architecture
//! ([ISA](https://en.wikipedia.org/wiki/Instruction_set_architecture)).
//!
//! This crate provides a toolkit for creating software implementations of the base 32-bit variant of the RISC-V ISA
//! and a number of its extensions, including:
//!
//! - the RV32I base integer instruction set
//! - the 'M' standard extension for integer multiplication and division
//! - the 'F' standard extension for single-precision floating point
//! - the 'C' standard extension for compressed instructions
//!

pub mod cpu;
pub mod cpu_types;
pub mod dispatcher;
pub mod disassembler;
pub mod memory;
pub mod tobits;
pub mod trap_handler;

pub use self::cpu::*;
pub use self::cpu_types::*;
pub use self::dispatcher::*;
pub use self::disassembler::*;
pub use self::memory::*;
pub use self::tobits::*;
pub use self::trap_handler::*;
