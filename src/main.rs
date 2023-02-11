use arviss_experiment::{decode, Decoder};
use arviss_experiment::{
    ExecFnBimm12Rs1Rs2::{self, *},
    ExecFnImm12RdRs1::{self, *},
    ExecFnImm12Rs1Rs2::{self, *},
    ExecFnImm20Rd, ExecFnJimm20Rd, ExecFnNoArgs, ExecFnRdFmPredRdRs1Succ,
    ExecFnRdRs1::{self, *},
    ExecFnRdRs1Rm,
    ExecFnRdRs1Rs2::{self, *},
    ExecFnRdRs1Rs2Rm, ExecFnRdRs1Rs2Rs3Rm,
    ExecFnRdRs1Shamtw::{self, *},
    ExecFnTrap,
};

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

fn extract_fm(ins: u32) -> u8 {
    ((ins >> 28) & 0xf) as u8
}

fn extract_rd(ins: u32) -> u8 {
    ((ins >> 7) & 0x1f) as u8
}

fn extract_rs1(ins: u32) -> u8 {
    ((ins >> 15) & 0x1f) as u8
}

fn extract_rs2(ins: u32) -> u8 {
    ((ins >> 20) & 0x1f) as u8
}

fn extract_rs3(ins: u32) -> u8 {
    ((ins >> 27) & 0x1f) as u8
}

fn extract_rm(ins: u32) -> u8 {
    ((ins >> 12) & 7) as u8
}

fn extract_bimmediate(ins: u32) -> i32 {
    let p0 = ((ins & 0x80000000) as i32) >> 19; // inst[31] -> sext(imm[12])
    let p1 = ((ins & 0x00000080) << 4) as i32; // inst[7] -> imm[11]
    let p2 = ((ins & 0x7e000000) >> 20) as i32; // inst[30:25] -> imm[10:5]
    let p3 = ((ins & 0x00000f00) >> 7) as i32; // inst[11:8]  -> imm[4:1]
    p0 | p1 | p2 | p3
}

fn extract_iimmediate(ins: u32) -> i32 {
    (ins as i32) >> 20 // inst[31:20] -> sext(imm[11:0])
}

fn extract_jimmediate(ins: u32) -> i32 {
    let p0 = ((ins & 0x80000000) as i32) >> 11; // inst[31] -> sext(imm[20])
    let p1 = (ins & 0x000ff000) as i32; // inst[19:12] -> imm[19:12]
    let p2 = ((ins & 0x00100000) >> 9) as i32; // inst[20] -> imm[11]
    let p3 = ((ins & 0x7fe00000) >> 20) as i32; // inst[20] -> imm[11]
    p0 | p1 | p2 | p3
}

fn extract_simmediate(ins: u32) -> i32 {
    let p0 = ((ins & 0xfe000000) as i32) >> 20; // inst[31:25] -> sext(imm[11:5])
    let p1 = ((ins & 0x00000f80) >> 7) as i32; // inst[11:7]  -> imm[4:0]
    p0 | p1
}

fn extract_uimmediate(ins: u32) -> i32 {
    (ins & 0xfffff000) as i32 // inst[31:12] -> imm[31:12]
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

#[derive(Debug)]
enum DecodedInstruction {
    NoArgs {
        opcode: ExecFnNoArgs, // Which opcodes are viable for these parameters.
    },
    Fdr {
        cache_line: u32, // The instruction's cache line.
        index: u32,      // The instruction's index in the cache line.
    },
    RdFmPredRdRs1Succ {
        opcode: ExecFnRdFmPredRdRs1Succ, // Which opcodes are viable for these parameters.
        fm: u8,                          // Fence "mode".
        rd: u8,                          // Destination register. Currently ignored.
        rs1: u8,                         // Source register. Currently ignored.
    },
    RdImm20 {
        opcode: ExecFnImm20Rd, // Which opcodes are viable for these parameters.
        rd: u8,                // Destination register.
        imm: i32,              // Immediate operand.
    },
    RdJImm20 {
        opcode: ExecFnJimm20Rd, // Which opcodes are viable for these parameters.
        rd: u8,                 // Destination register.
        imm: i32,               // Immediate operand.
    },
    RdRs1 {
        opcode: ExecFnRdRs1, // Which opcodes are viable for these parameters.
        rd: u8,              // Destination register.
        rs1: u8,             // Source register.
    },
    RdRs1Imm12 {
        opcode: ExecFnImm12RdRs1, // Which opcodes are viable for these parameters.
        rd: u8,                   // Destination register.
        rs1: u8,                  // Source register.
        imm: i32,                 // Immediate operand.
    },
    RdRs1Shamtw {
        opcode: ExecFnRdRs1Shamtw, // Which opcodes are viable for these parameters.
        rd: u8,                    // Destination register.
        rs1: u8,                   // Source register.
        imm: i32,                  // Immediate operand.
    },
    RdRs1Rs2 {
        opcode: ExecFnRdRs1Rs2, // Which opcodes are viable for these parameters.
        rd: u8,                 // Destination register.
        rs1: u8,                // First source register.
        rs2: u8,                // Second source register.
    },
    Rs1Rs2Imm12 {
        opcode: ExecFnImm12Rs1Rs2, // Which opcodes are viable for these parameters.
        rs1: u8,                   // First source register.
        rs2: u8,                   // Second source register.
        imm: i32,                  // Immediate operand.
    },
    Rs1Rs2BImm12 {
        opcode: ExecFnBimm12Rs1Rs2, // Which opcodes are viable for these parameters.
        rs1: u8,                    // First source register.
        rs2: u8,                    // Second source register.
        imm: i32,                   // Immediate operand.
    },
    RdRs1Rs2Rs3Rm {
        opcode: ExecFnRdRs1Rs2Rs3Rm, // Which opcodes are viable for these parameters.
        rd: u8,                      // Destination register.
        rs1: u8,                     // First source register.
        rs2: u8,                     // Second source register.
        rs3: u8,                     // Third source register.
        rm: u8,
    },
    RdRs1Rm {
        opcode: ExecFnRdRs1Rm, // Which opcodes are viable for these parameters.
        rd: u8,                // Destination register.
        rs1: u8,               // Source register.
        rm: u8,                // Rounding mode.
    },
    RdRs1Rs2Rm {
        opcode: ExecFnRdRs1Rs2Rm, // Which opcodes are viable for these parameters.
        rd: u8,                   // Destination register.
        rs1: u8,                  // First source register.
        rs2: u8,                  // Second source register.
        rm: u8,                   // Rounding mode.
    },
    Ins {
        opcode: ExecFnTrap, // Which opcodes are viable for these parameters.
        ins: u32,
    },
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

struct Generator;

impl Decoder for Generator {
    type Item = DecodedInstruction;

    fn gen_trap(&mut self, opcode: ExecFnTrap, ins: u32) -> DecodedInstruction {
        DecodedInstruction::Ins { opcode, ins }
    }

    fn gen_no_args(&mut self, opcode: ExecFnNoArgs, _ins: u32) -> DecodedInstruction {
        DecodedInstruction::NoArgs { opcode }
    }

    fn gen_jimm20_rd(&mut self, opcode: ExecFnJimm20Rd, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdJImm20 {
            opcode: opcode,
            rd: extract_rd(ins),
            imm: extract_jimmediate(ins),
        }
    }

    fn gen_bimm12hi_bimm12lo_rs1_rs2(
        &mut self,
        opcode: ExecFnBimm12Rs1Rs2,
        ins: u32,
    ) -> DecodedInstruction {
        DecodedInstruction::Rs1Rs2BImm12 {
            opcode,
            rs1: extract_rs1(ins),
            rs2: extract_rs2(ins),
            imm: extract_bimmediate(ins),
        }
    }

    fn gen_rd_rm_rs1(&mut self, opcode: ExecFnRdRs1Rm, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Rm {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            rm: extract_rm(ins),
        }
    }

    fn gen_rd_rm_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2Rm, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Rs2Rm {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            rs2: extract_rs2(ins),
            rm: extract_rm(ins),
        }
    }

    fn gen_rd_rs1(&mut self, opcode: ExecFnRdRs1, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1 {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
        }
    }

    fn gen_rd_rm_rs1_rs2_rs3(
        &mut self,
        opcode: ExecFnRdRs1Rs2Rs3Rm,
        ins: u32,
    ) -> DecodedInstruction {
        DecodedInstruction::RdRs1Rs2Rs3Rm {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            rs2: extract_rs2(ins),
            rs3: extract_rs3(ins),
            rm: extract_rm(ins),
        }
    }

    fn gen_rd_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Rs2 {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            rs2: extract_rs2(ins),
        }
    }

    fn gen_imm12hi_imm12lo_rs1_rs2(
        &mut self,
        opcode: ExecFnImm12Rs1Rs2,
        ins: u32,
    ) -> DecodedInstruction {
        DecodedInstruction::Rs1Rs2Imm12 {
            opcode,
            rs1: extract_rs1(ins),
            rs2: extract_rs2(ins),
            imm: extract_simmediate(ins),
        }
    }

    fn gen_imm20_rd(&mut self, opcode: ExecFnImm20Rd, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdImm20 {
            opcode,
            rd: extract_rd(ins),
            imm: extract_uimmediate(ins),
        }
    }

    fn gen_rd_rs1_shamtw(&mut self, opcode: ExecFnRdRs1Shamtw, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Shamtw {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            imm: extract_iimmediate(ins),
        }
    }

    fn gen_fm_pred_rd_rs1_succ(
        &mut self,
        opcode: ExecFnRdFmPredRdRs1Succ,
        ins: u32,
    ) -> DecodedInstruction {
        DecodedInstruction::RdFmPredRdRs1Succ {
            opcode,
            fm: extract_fm(ins),
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
        }
    }

    fn gen_imm12_rd_rs1(&mut self, opcode: ExecFnImm12RdRs1, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Imm12 {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            imm: extract_iimmediate(ins),
        }
    }
}

struct Disassembler {}

impl Disassembler {
    const ABI_NAMES: &[&'static str] = &[
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
        "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
        "t5", "t6",
    ];

    const FABI_NAMES: &[&'static str] = &[
        "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "fs0", "fs1", "fa0", "fa1", "fa2",
        "fa3", "fa4", "fa5", "fa6", "fa7", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7", "fs8", "fs9",
        "fs10", "fs11", "ft8", "ft9", "ft10", "ft11",
    ];

    const ROUNDING_MODES: &[&'static str] = &[
        "rne",
        "rtz",
        "rdn",
        "rup",
        "rmm",
        "reserved5",
        "reserved6",
        "dyn",
    ];

    fn abi_name(reg: u8) -> &'static str {
        match reg {
            0..=31 => Disassembler::ABI_NAMES[reg as usize],
            _ => unreachable!(),
        }
    }

    fn fabi_name(reg: u8) -> &'static str {
        match reg {
            0..=31 => Disassembler::FABI_NAMES[reg as usize],
            _ => unreachable!(),
        }
    }

    fn rounding_mode(mode: u8) -> &'static str {
        match mode {
            0..=7 => Disassembler::ROUNDING_MODES[mode as usize],
            _ => unreachable!(),
        }
    }
}

impl Decoder for Disassembler {
    type Item = String;

    fn gen_trap(&mut self, opcode: ExecFnTrap, ins: u32) -> Self::Item {
        // Illegal instruction.
        format!("{}\t0x{:04x}", opcode, ins)
    }

    fn gen_no_args(&mut self, opcode: ExecFnNoArgs, _ins: u32) -> Self::Item {
        // "ECALL"
        // "EBREAK"
        // "URET"
        // "SRET"
        // "MRET"
        format!("{}", opcode)
    }

    fn gen_jimm20_rd(&mut self, opcode: ExecFnJimm20Rd, ins: u32) -> Self::Item {
        // "JAL %s, %d", abiNames[ins->rd_imm.rd], ins->rd_imm.imm
        format!(
            "{}\t{}, {}",
            opcode,
            extract_rd(ins),
            extract_jimmediate(ins)
        )
    }

    fn gen_bimm12hi_bimm12lo_rs1_rs2(
        &mut self,
        opcode: ExecFnBimm12Rs1Rs2,
        ins: u32,
    ) -> Self::Item {
        match opcode {
            // "BEQ %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BNE %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BLT %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BGE %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BLTU %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BGEU %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            ExecBeq | ExecBne | ExecBlt | ExecBge | ExecBltu | ExecBgeu => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rs1(ins)),
                    Disassembler::abi_name(extract_rs2(ins)),
                    extract_bimmediate(ins)
                )
            }
        }
    }

    fn gen_rd_rm_rs1(&mut self, opcode: ExecFnRdRs1Rm, ins: u32) -> Self::Item {
        // "FSQRT.S %s, %s, %s", fabiNames[ins->rd_rs1_rm.rd], fabiNames[ins->rd_rs1_rm.rs1], roundingModes[ins->rd_rs1_rm.rm])
        // "FCVT.W.S %s, %s, %s", abiNames[ins->rd_rs1_rm.rd], fabiNames[ins->rd_rs1_rm.rs1], roundingModes[ins->rd_rs1_rm.rm])
        // "FCVT.WU.S %s, %s, %s", abiNames[ins->rd_rs1_rm.rd], fabiNames[ins->rd_rs1_rm.rs1], roundingModes[ins->rd_rs1_rm.rm])
        // "FCVT.S.W %s, %s, %s", fabiNames[ins->rd_rs1_rm.rd], abiNames[ins->rd_rs1_rm.rs1], roundingModes[ins->rd_rs1_rm.rm])
        // "FVCT.S.WU %s, %s, %s", fabiNames[ins->rd_rs1_rm.rd], abiNames[ins->rd_rs1_rm.rs1], roundingModes[ins->rd_rs1_rm.rm])
        format!(
            "{}\t{}, {}, {}",
            opcode,
            Disassembler::fabi_name(extract_rd(ins)),
            Disassembler::fabi_name(extract_rs1(ins)),
            Disassembler::rounding_mode(extract_rm(ins)),
        )
    }

    fn gen_rd_rm_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2Rm, ins: u32) -> Self::Item {
        // "FADD.S %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rm.rd], fabiNames[ins->rd_rs1_rs2_rm.rs1], fabiNames[ins->rd_rs1_rs2_rm.rs2], roundingModes[ins->rd_rs1_rs2_rm.rm])
        // "FSUB.S %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rm.rd], fabiNames[ins->rd_rs1_rs2_rm.rs1], fabiNames[ins->rd_rs1_rs2_rm.rs2], roundingModes[ins->rd_rs1_rs2_rm.rm])
        // "FMUL.S %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rm.rd], fabiNames[ins->rd_rs1_rs2_rm.rs1], fabiNames[ins->rd_rs1_rs2_rm.rs2], roundingModes[ins->rd_rs1_rs2_rm.rm])
        // "FDIV.S %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rm.rd], fabiNames[ins->rd_rs1_rs2_rm.rs1], fabiNames[ins->rd_rs1_rs2_rm.rs2], roundingModes[ins->rd_rs1_rs2_rm.rm])
        format!(
            "{}\t{}, {}, {}, {}",
            opcode,
            Disassembler::fabi_name(extract_rd(ins)),
            Disassembler::fabi_name(extract_rs1(ins)),
            Disassembler::fabi_name(extract_rs2(ins)),
            Disassembler::rounding_mode(extract_rm(ins))
        )
    }

    fn gen_rd_rs1(&mut self, opcode: ExecFnRdRs1, ins: u32) -> Self::Item {
        // "FMV.X.W %s, %s", abiNames[ins->rd_rs1.rd], fabiNames[ins->rd_rs1.rs1])
        // "FMV.W.X %s, %s", fabiNames[ins->rd_rs1.rd], abiNames[ins->rd_rs1.rs1])
        // "FCLASS.S %s, %s", abiNames[ins->rd_rs1.rd], fabiNames[ins->rd_rs1.rs1])
        match opcode {
            ExecFmvXW | ExecFclassS => {
                format!(
                    "{}\t{}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::fabi_name(extract_rs1(ins))
                )
            }
            ExecFmvWX => {
                format!(
                    "{}\t{}, {}",
                    opcode,
                    Disassembler::fabi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins))
                )
            }
        }
    }

    fn gen_rd_rm_rs1_rs2_rs3(&mut self, opcode: ExecFnRdRs1Rs2Rs3Rm, ins: u32) -> Self::Item {
        // "FMADD.S %s, %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rs3_rm.rd], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs1], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs2], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs3], roundingModes[ins->rd_rs1_rs2_rs3_rm.rm])
        // "FMSUB.S %s, %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rs3_rm.rd], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs1], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs2], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs3], roundingModes[ins->rd_rs1_rs2_rs3_rm.rm])
        // "FNMSUB.S %s, %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rs3_rm.rd], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs1], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs2], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs3], roundingModes[ins->rd_rs1_rs2_rs3_rm.rm])
        // "FNMADD.S %s, %s, %s, %s, %s", fabiNames[ins->rd_rs1_rs2_rs3_rm.rd], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs1], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs2], fabiNames[ins->rd_rs1_rs2_rs3_rm.rs3], roundingModes[ins->rd_rs1_rs2_rs3_rm.rm])
        format!(
            "{}\t{}, {}, {}, {}, {}",
            opcode,
            Disassembler::fabi_name(extract_rd(ins)),
            Disassembler::fabi_name(extract_rs1(ins)),
            Disassembler::fabi_name(extract_rs2(ins)),
            Disassembler::fabi_name(extract_rs3(ins)),
            Disassembler::rounding_mode(extract_rm(ins))
        )
    }

    fn gen_rd_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2, ins: u32) -> Self::Item {
        match opcode {
            // "ADD %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "MUL %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "SUB %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "SLL %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "MULH %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "SLT %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "MULHSU %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "SLTU %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "MULHU %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "XOR %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "DIV %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "SRL %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "DIVU %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "SRA %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "OR %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "REM %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "AND %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            // "REMU %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], abiNames[ins->rd_rs1_rs2.rs1], abiNames[ins->rd_rs1_rs2.rs2]
            ExecAdd | ExecMul | ExecSub | ExecSll | ExecMulh | ExecSlt | ExecMulhsu | ExecSltu
            | ExecMulhu | ExecXor | ExecDiv | ExecSrl | ExecDivu | ExecSra | ExecOr | ExecRem
            | ExecAnd | ExecRemu => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins)),
                    Disassembler::abi_name(extract_rs2(ins))
                )
            }
            // "FLE.S %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            // "FLT.S %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            // "FEQ.S %s, %s, %s", abiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            ExecFleS | ExecFltS | ExecFeqS => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::fabi_name(extract_rs1(ins)),
                    Disassembler::fabi_name(extract_rs2(ins))
                )
            }
            // "FSGNJ.S %s, %s, %s", fabiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            // "FMIN.S %s, %s, %s", fabiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            // "FSGNJN.S %s, %s, %s", fabiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            // "FMAX.S %s, %s, %s", fabiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            // "FSGNJX.S %s, %s, %s", fabiNames[ins->rd_rs1_rs2.rd], fabiNames[ins->rd_rs1_rs2.rs1], fabiNames[ins->rd_rs1_rs2.rs2])
            ExecFsgnjS | ExecFminS | ExecFsgnjnS | ExecFmaxS | ExecFsgnjxS => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::fabi_name(extract_rd(ins)),
                    Disassembler::fabi_name(extract_rs1(ins)),
                    Disassembler::fabi_name(extract_rs2(ins))
                )
            }
        }
    }

    fn gen_imm12hi_imm12lo_rs1_rs2(&mut self, opcode: ExecFnImm12Rs1Rs2, ins: u32) -> Self::Item {
        match opcode {
            // "SB %s, %d(%s)", abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            // "SH %s, %d(%s)", abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            // "SW %s, %d(%s)", abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            ExecSb | ExecSh | ExecSw => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::abi_name(extract_rs2(ins)),
                    extract_bimmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }

            // "FSW %s, %d(%s)", fabiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            ExecFsw => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::fabi_name(extract_rs2(ins)),
                    extract_bimmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }
        }
    }

    fn gen_imm20_rd(&mut self, opcode: ExecFnImm20Rd, ins: u32) -> Self::Item {
        // TRACE("AUIPC %s, %d\n", abiNames[ins->rd_imm.rd], ins->rd_imm.imm >> 12);
        // TRACE("LUI %s, %d\n", abiNames[ins->rd_imm.rd], ins->rd_imm.imm >> 12);
        format!(
            "{}\t{}, {}",
            opcode,
            Disassembler::abi_name(extract_rd(ins)),
            extract_uimmediate(ins) >> 12 // TODO: Does the shift belong here, or with extract_uimmediate()?
        )
    }

    fn gen_rd_rs1_shamtw(&mut self, opcode: ExecFnRdRs1Shamtw, ins: u32) -> Self::Item {
        match opcode {
            // "SLLI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SRLI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SRAI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            ExecSlli | ExecSrli | ExecSrai => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins)),
                    extract_iimmediate(ins)
                )
            }
        }
    }

    fn gen_fm_pred_rd_rs1_succ(
        &mut self,
        opcode: ExecFnRdFmPredRdRs1Succ,
        _ins: u32,
    ) -> Self::Item {
        // "FENCE"
        // We're totally ignoring FENCE.TSO as it's optional.
        format!("{}\t", opcode)
    }

    fn gen_imm12_rd_rs1(&mut self, opcode: ExecFnImm12RdRs1, ins: u32) -> Self::Item {
        match opcode {
            // "LB %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LH %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LW %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LBU %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LHU %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            ExecLb | ExecLh | ExecLw | ExecLbu | ExecLhu => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    extract_iimmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }
            // "FLW %s, %d(%s)", fabiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            ExecFlw => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::fabi_name(extract_rd(ins)),
                    extract_iimmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }
            // TODO:
            ExecFenceI => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    extract_rd(ins),
                    extract_rs1(ins),
                    extract_iimmediate(ins)
                )
            }
            // "ADDI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SLTI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SLTIU %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "XORI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "ORI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "ANDI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            ExecAddi | ExecSlti | ExecSltiu | ExecXori | ExecOri | ExecAndi => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins)),
                    extract_iimmediate(ins)
                )
            }
            // "JALR %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            ExecJalr => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins)),
                    extract_iimmediate(ins),
                )
            }
        }
    }
}
