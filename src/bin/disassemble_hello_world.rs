use std::fs::File;
use std::io::prelude::*;

use arviss_experiment::disassembler::Disassembler;
use arviss_experiment::prelude::*;

pub fn main() {
    // Load an RV32IC image into a buffer.
    let mut f = File::open("images/hello_world.rv32ic").expect("Failed to open image.");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Failed to load image.");

    // Disassemble the image, one instruction at a time, taking into account that compact
    // instructions are only 2 bytes.
    let mut disassembler = Disassembler {};
    let mut index: usize = 0;
    let image = buffer.as_slice();
    println!("addr     instr    code");
    while index < image.len() - 4 {
        if let Ok(slice) = &image[index..index + 4].try_into() {
            let word = u32::from_le_bytes(*slice);
            let is_compact = (word & 3) != 3;
            let word = if is_compact { word & 0xffff } else { word };
            let result = disassembler.dispatch_rv32ic(word);
            if is_compact {
                // Compact instructions are 2 bytes each.
                println!("{:08x}     {:04x} {}", index, word, result);
                index = index + 2;
            } else {
                // Regular instructions are 4 bytes each.
                println!("{:08x} {:08x} {}", index, word, result);
                index = index + 4;
            }
        }
    }
}
