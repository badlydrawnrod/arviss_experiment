use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use arviss::Rv32iDispatcher;

use arviss::disassembler::Disassembler;
use arviss::platforms::basic::*;

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

    // Create a simulator and copy the image from the buffer into simulator memory.
    let mut cpu = Rv32iCpu::<BasicMem>::new();
    cpu.write_bytes(0, buffer.as_slice())
        .expect("Failed to initialize memory.");

    // Run until we can run no more.
    let mut disassembler = Disassembler {};
    if disassemble {
        println!("pc       (pc)     Code");
    }
    while !cpu.is_trapped() {
        // Fetch.
        let ins = cpu.fetch().unwrap();

        // Disassemble if the user asked for it.
        if disassemble {
            let result = disassembler.dispatch(ins);
            println!("{:08x} {:08x} {}", cpu.pc(), ins, result);
        }

        // Decode and dispatch.
        cpu.dispatch(ins);
    }

    match cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {}
        Some(cause) => println!("{:?} at 0x{:08x}", cause, cpu.pc()),
        None => {}
    }

    Ok(())
}
