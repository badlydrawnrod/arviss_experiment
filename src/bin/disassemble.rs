use arviss_experiment::{decode2, Disassembler};

pub fn main() {
    let mut disassembler = Disassembler {};

    for ins in [
        // _start:
        0x00_00_51_97, // auipc gp, 5
        0x80_01_81_93, // addi  gp, gp, -2048
        // .Lpcrel_hi1:
        0x00_00_81_17, // auipc sp, 8
        0xff_81_01_13, // addi  sp, sp, -8
        0x00_01_04_33, // add   s0, sp, zero
        // .Lpcrel_hi2:
        0x00_00_45_17, // auipc a0, 4
        0xfe_c5_05_13, // addi  a0, a0, -20
        // .Lpcrel_hi3:
        0x00_00_45_97, // auipc a1, 4
        0xfe_45_85_93, // addi  a1, a1, -28
        0x00_00_06_13, // mv    a2, zero
        // clear_bss:
        0x00_b5_78_63, // bgeu  a0, a1, 16
        0x00_c5_00_23, // sb    a2, 0(a0)
        0x00_15_05_13, // addi  a0, a0, 1
        0xfe_00_0a_e3, // beqz  zero, -12
        // finish_bss:
        0x00_00_00_97, // auipc ra, 0
        0x00_c0_80_e7, // jalr  12(ra)
        0x00_10_00_73, // ebreak
    ] {
        let result = decode2(&mut disassembler, ins);
        println!("{:08x} {}", ins, result);
    }
}
