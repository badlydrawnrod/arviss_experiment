use arviss_experiment::{decode, Disassembler};

pub fn main() {
    let mut dissassembler = Disassembler {};

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
        let result = decode(&mut dissassembler, ins);
        println!("{:08x} {}", ins, result);
    }
}

// TODO: These exist for *testing*. Move them into a testing module.
#[derive(Debug)]
enum ArvissOpcode {
    OpLUI = 0b0110111,
    OpAUIPC = 0b0010111,
    OpJAL = 0b1101111,
    OpJALR = 0b1100111,
    OpBRANCH = 0b1100011,
    OpLOAD = 0b0000011,
    OpSTORE = 0b0100011,
    OpOPIMM = 0b0010011,
    OpOP = 0b0110011,
    OpMISCMEM = 0b0001111,
    OpSYSTEM = 0b1110011,
    OpLOADFP = 0b0000111,  // RV32F
    OpSTOREFP = 0b0100111, // RV32F
    OpOPFP = 0b1010011,    // RV32F
    OpMADD = 0b1000011,    // RV32F
    OpMSUB = 0b1000111,    // RV32F
    OpNMSUB = 0b1001011,   // RV32F
    OpNMADD = 0b1001111,   // RV32F
}

trait Dispatcher {
    fn exec_add(&mut self, rd: u8, rs1: u8, imm: i32);
    fn exec_ecall(&mut self);
}

// impl Dispatcher for Disassembler {
//     fn exec_add(&mut self, rd: u8, rs1: u8, imm: i32) {
//         println!("add {}, {}, {}", rd, rs1, imm);
//     }

//     fn exec_ecall(&mut self) {
//         println!("ecall");
//     }
// }

// fn dispatch(dispatcher: &mut impl Dispatcher, ins: DecodedInstruction) {
//     match ins {
//         DecodedInstruction::RdRs1Imm {
//             opcode: ExecAddi,
//             rd,
//             rs1,
//             imm,
//         } => dispatcher.exec_add(rd, rs1, imm),
//         DecodedInstruction::NoArgs { opcode: ExecEcall } => dispatcher.exec_ecall(),
//         _ => {}
//     }
// }
