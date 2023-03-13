use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use arviss_experiment::{decode, BasicMem, CoreCpu, Disassembler, Loader, Rv32iCpu};

const EBREAK: u32 = 0x00_10_00_73;

pub fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let (disassemble, filename) = match args.len() {
        2 => (false, &args[1]),
        3 if args[1] == "-d" => (true, &args[2]),
        _ => {
            eprintln!("\nUsage:\n\t{} [-d] <filename>", args[0]);
            std::process::exit(2);
        }
    };

    // Load the image into a buffer.
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Copy the image from the buffer into simulator memory.
    let mut mem = BasicMem::new();
    let ins = buffer.as_slice();
    mem.write_bytes(0, ins)
        .expect("Failed to initialize memory.");

    // Create a CPU that will use that memory.
    let mut cpu = Rv32iCpu::<BasicMem>::with_mem(mem);

    // Run until we can run no more.
    let mut disassembler = Disassembler {};
    if disassemble {
        println!("pc       (pc)     Code");
    }
    loop {
        // Fetch.
        let pc = cpu.get_next_pc();
        cpu.wpc(pc);
        cpu.set_next_pc(pc.wrapping_add(4));
        let ins = cpu.read32(pc).unwrap();

        // Disassemble if the user asked for it.
        if disassemble {
            let result = decode(&mut disassembler, ins);
            println!("{:08x} {:08x} {}", pc, ins, result);
        }
        if ins == EBREAK {
            break;
        }

        // Decode and execute.
        decode(&mut cpu, ins);
    }

    Ok(())
}
