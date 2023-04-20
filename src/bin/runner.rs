use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use arviss_experiment::prelude::*;

// A shim that makes it easy to change dispatchers.
#[inline]
fn dispatch<T>(dispatcher: &mut impl Rv32iDispatcher<Item = T>, code: u32) -> T {
    dispatcher.dispatch_rv32i(code)
}

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
    while !cpu.is_trapped() {
        // Fetch.
        let ins = cpu.fetch().unwrap();

        // Disassemble if the user asked for it.
        if disassemble {
            let result = dispatch(&mut disassembler, ins);
            println!("{:08x} {:08x} {}", cpu.get_pc(), ins, result);
        }

        // Decode and dispatch.
        dispatch(&mut cpu, ins);
    }

    match cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {}
        Some(cause) => println!("{:?} at 0x{:08x}", cause, cpu.get_pc()),
        None => {}
    }

    Ok(())
}
