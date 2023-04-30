use std::fs::File;
use std::io::prelude::*;

use arviss::Rv32iDispatcher;

use arviss::profiles::basic_mem::*;
use arviss::profiles::rv32icpu::*;

pub fn main() {
    // Load the image into a buffer.
    let mut f = File::open("images/hello_world.rv32i").expect("Failed to open image.");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Failed to load image.");

    // Copy the image into simulator memory.
    let mut mem = BasicMem::new();
    let image = buffer.as_slice();
    mem.write_bytes(0, image)
        .expect("Failed to initialize memory.");

    // Execute the image.
    let mut cpu = Rv32iCpu::<BasicMem>::with_mem(mem);
    while !cpu.is_trapped() {
        let instruction = cpu.fetch().expect("Failed to fetch instruction.");
        cpu.dispatch(instruction);
    }
}
