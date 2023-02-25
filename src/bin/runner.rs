use arviss_experiment::{decode, BasicCpu, BasicMem, Disassembler, Loader, Mem};

pub fn main() {
    let ins: &[u8] = &[
        // _start:
        0x97, 0x51, 0x00, 0x00, // auipc gp, 5
        0x93, 0x81, 0x01, 0x80, // addi  gp, gp, -2048
        // .Lpcrel_hi1:
        0x17, 0x81, 0x00, 0x00, // auipc sp, 8
        0x13, 0x01, 0x81, 0xff, // addi  sp, sp, -8
        0x33, 0x04, 0x01, 0x00, // add   s0, sp, zero
        // .Lpcrel_hi2:
        0x17, 0x45, 0x00, 0x00, // auipc a0, 4
        0x13, 0x05, 0xc5, 0xfe, // addi  a0, a0, -20
        // .Lpcrel_hi3:
        0x97, 0x45, 0x00, 0x00, // auipc a1, 4
        0x93, 0x85, 0x45, 0xfe, // addi  a1, a1, -28
        0x13, 0x06, 0x00, 0x00, // mv    a2, zero
        // clear_bss:
        0x63, 0x78, 0xb5, 0x00, // bgeu  a0, a1, 16
        0x23, 0x00, 0xc5, 0x00, // sb    a2, 0(a0)
        0x13, 0x05, 0x15, 0x00, // addi  a0, a0, 1
        0xe3, 0x0a, 0x00, 0xfe, // beqz  zero, -12
        // finish_bss:
        0x97, 0x00, 0x00, 0x00, // auipc ra, 0
        0x73, 0x00, 0x10, 0x00, // ebreak
    ];

    let mut mem = BasicMem::new();
    mem.write_bytes(0, ins)
        .expect("Failed to initialize memory.");

    let mut disassembler = Disassembler {};
    let mut cpu = BasicCpu::with_mem(mem);

    const EBREAK: u32 = 0x00_10_00_73;
    println!("pc       (pc)     Code");
    loop {
        // Fetch.
        let pc = cpu.pc;
        let ins = cpu.mem.read32(pc).unwrap();
        
        // Disassemble.
        let result = decode(&mut disassembler, ins);
        println!("{:08x} {:08x} {}", pc, ins, result);
        if ins == EBREAK {
            break;
        }
        
        // Decode and execute.
        decode(&mut cpu, ins);
    }
}
