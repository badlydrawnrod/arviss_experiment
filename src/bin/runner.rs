use std::fs::File;
use std::io;
use std::io::prelude::*;

use arviss_experiment::{decode, BasicCpu, BasicMem, Disassembler, Loader, Mem};

const EBREAK: u32 = 0x00_10_00_73;

pub fn main() -> io::Result<()> {
    // Load the image into a buffer.
    let mut f = File::open("../rt_app/app.bin")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Copy the image from the buffer into simulator memory.
    let mut mem = BasicMem::new();
    let ins = buffer.as_slice();
    mem.write_bytes(0, ins)
        .expect("Failed to initialize memory.");

    // Create a CPU that will use that memory.
    let mut cpu = BasicCpu::with_mem(mem);

    // Run until we can run no more.
    let mut disassembler = Disassembler {};
    println!("pc       (pc)     Code");
    loop {
        // Fetch.
        let pc = cpu.pc;
        let ins = cpu.mem.read32(pc).unwrap();

        // Disassemble.
        // TODO: toggle via command line
        // let result = decode(&mut disassembler, ins);
        // println!("{:08x} {:08x} {}", pc, ins, result);
        if ins == EBREAK {
            break;
        }

        // Decode and execute.
        decode(&mut cpu, ins);
    }

    Ok(())
}
