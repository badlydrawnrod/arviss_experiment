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
//! To do this, it loads the data from an image and uses it to populate a [`platforms::basic::BasicCpu`]'s memory. It
//! then executes instructions, dispatching them with [`Rv32iDispatcher`] which is implemented for [`Rv32iHandler`].
//!
//! It does this until the CPU hits a trap, which it will do when it reaches an `ebreak`.
//!
//! If you run this example it should output "Hello, world from Rust!" multiple times.
//!
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! use arviss::Rv32iDispatcher;
//!
//! use arviss::platforms::basic::*;
//!
//! // Load an RV32I image into a buffer.
//! let mut f = File::open("images/hello_world.rv32i")?;
//! let mut buffer = Vec::new();
//! f.read_to_end(&mut buffer)?;
//!
//! // Create a simulator and copy the image from the buffer into simulator memory.
//! let mut cpu = Rv32iCpu::<BasicMem>::new();
//! cpu.write_bytes(0, buffer.as_slice())
//!     .expect("Failed to initialize memory.");
//!
//! // Execute the image.
//! while !cpu.is_trapped() {
//!     let instruction = cpu.fetch().unwrap();
//!     cpu.dispatch(instruction);
//! }
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! This example loads a binary RV32IC image and disassembles it.
//!
//! ```
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! use arviss::Rv32icDispatcher;
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
//!         let result = disassembler.dispatch(word);
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

pub mod backends;
pub mod disassembler;
pub mod platforms;
pub mod reg;
pub mod tobits;

mod cpu;
mod dispatcher;
mod handlers;
mod memory;
mod trap;

#[doc(inline)]
pub use cpu::*;

#[doc(inline)]
pub use dispatcher::*;

#[doc(inline)]
pub use handlers::*;

#[doc(inline)]
pub use memory::*;

#[doc(inline)]
pub use trap::*;
