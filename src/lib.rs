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
//! This example loads a binary RV32I image into simulator memory then executes it.
//!
//! To do this, it loads the data from an image and uses it to populate a [`profiles::memory::BasicMem`] memory
//! implementation. It then creates an [`profiles::cpu::Rv32iCpu`] using that memory, then executes instructions from that
//! memory by fetching them then dispatching them with [`Rv32iDispatcher`] which is implemented for
//! [`Rv32i`].
//!
//! It does this until the CPU hits a trap, which it will do when it reaches an `ebreak`.
//!
//! If you run this example it should output "Hello, world from Rust!" multiple times.
//!
//! ```
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! use arviss::prelude::*;
//!
//! use arviss::profiles::memory::BasicMem;
//! use arviss::profiles::cpu::Rv32iCpu;
//!
//! // Load an RV32I image into a buffer.
//! let mut f = File::open("images/hello_world.rv32i").expect("Failed to open image.");
//! let mut buffer = Vec::new();
//! f.read_to_end(&mut buffer).expect("Failed to load image.");
//!
//! // Copy the image into simulator memory.
//! let mut mem = BasicMem::new();
//! let image = buffer.as_slice();
//! mem.write_bytes(0, image).expect("Failed to initialize memory.");
//!
//! // Execute the image.
//! let mut cpu = Rv32iCpu::<BasicMem>::with_mem(mem);
//! while !cpu.is_trapped() {
//!     let instruction = cpu.fetch().expect("Failed to fetch instruction.");
//!     cpu.dispatch_rv32i(instruction);
//! }
//! ```
//!
//! This example loads a binary RV32IC image and disassembles it.
//!
//! ```
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! use arviss::prelude::*;
//!
//! use arviss::disassembler::Disassembler;
//!
//! // Load an RV32IC image into a buffer.
//! let mut f = File::open("images/hello_world.rv32ic").expect("Failed to open image.");
//! let mut buffer = Vec::new();
//! f.read_to_end(&mut buffer).expect("Failed to load image.");
//!
//! // Disassemble the image, one instruction at a time, taking into account that compact
//! // instructions are only 2 bytes.
//! let mut disassembler = Disassembler {};
//! let mut index: usize = 0;
//! let image = buffer.as_slice();
//! println!("addr     instr    code");
//! while index < image.len() - 4 {
//!     if let Ok(slice) = &image[index..index + 4].try_into() {
//!         let word = u32::from_le_bytes(*slice);
//!         let is_compact = (word & 3) != 3;
//!         let word = if is_compact { word & 0xffff } else { word };
//!         let result = disassembler.dispatch_rv32ic(word);
//!         if is_compact {
//!             // Compact instructions are 2 bytes each.
//!             println!("{:08x}     {:04x} {}", index, word, result);
//!             index += 2;
//!         } else {
//!             // Regular instructions are 4 bytes each.
//!             println!("{:08x} {:08x} {}", index, word, result);
//!             index += 4;
//!         }
//!     }
//! }
//! ```

pub mod prelude;

pub mod cpu;
pub mod disassembler;
pub mod dispatcher;
pub mod memory;
pub mod profiles;
pub mod reg;
pub mod tobits;
pub mod trap;

#[doc(inline)]
pub use cpu::*;

#[doc(inline)]
pub use dispatcher::*;

#[doc(inline)]
pub use memory::*;

#[doc(inline)]
pub use trap::*;
