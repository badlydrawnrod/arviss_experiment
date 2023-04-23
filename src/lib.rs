//! # Arviss
//!
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
//!
//! ## Examples
//! This example uses the [`basic_mem::BasicMem`] memory map with an [`rv32icpu::Rv32iCpu`]. It fetches instructions
//! from memory and dispatches them with [`dispatcher::Rv32iDispatcher`] which is implemented for [`cpu_types::Rv32i`].
//! It does this until the CPU hits a trap.
//! ```rust
//! use arviss_experiment::prelude::*;
//! use arviss_experiment::basic_mem::BasicMem;
//! use arviss_experiment::rv32icpu::Rv32iCpu;
//!
//! let mut mem = BasicMem::new();
//! let mut cpu = Rv32iCpu::<BasicMem>::with_mem(mem);
//! while !cpu.is_trapped() {
//!     let instruction = cpu.fetch().unwrap();
//!     cpu.dispatch_rv32i(instruction);
//! }
//! ```

pub mod prelude;

pub mod basic_mem;
pub mod cpu_types;
pub mod disassembler;
pub mod dispatcher;
pub mod memory;
pub mod reg;
pub mod rv32icpu;
pub mod tobits;
pub mod trap_handler;
