use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use arviss_experiment::{decode, BasicMem, CoreCpu, Disassembler, Loader, Rv32iCpu};

const EBREAK: u32 = 0x00_10_00_73;
const C_EBREAK: u32 = 0x9002;

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
        let ins = cpu.fetch().unwrap();

        // Disassemble if the user asked for it.
        if disassemble {
                let result = decode(&mut disassembler, ins);
                println!("{:08x} {:08x} {}", cpu.get_pc(), ins, result);
            // println!("{:08x} {:08x}", cpu.get_pc(), ins);
        }
        if ins == EBREAK || ins == C_EBREAK {
            break;
        }

        // Decode and execute.
        decode(&mut cpu, ins);
    }

    Ok(())
}
