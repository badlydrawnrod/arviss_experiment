// use std::io::{self, BufRead};
use std::fmt;

pub fn main() {
    // let stdin = io::stdin();
    // let line = stdin
    //     .lock()
    //     .lines()
    //     .next()
    //     .expect("there was no next line")
    //     .expect("the line could not be read");
    // let ins: u32 = line.parse().unwrap();

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
        println!("Code: {:08x} {}", ins, result);
    }
}

fn bits(n: u32, hi: u32, lo: u32) -> u32 {
    let run = (hi - lo) + 1;
    let mask = ((1 << run) - 1) << lo;
    (n & mask) >> lo
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
enum ExecFnCacheLineIndex {
    ExecFetchDecodeReplace,
}

#[derive(Debug)]
enum ExecFnNoArgs {
    ExecEcall,
    ExecEbreak,
    ExecUret,
    ExecSret,
    ExecMret,
}

impl fmt::Display for ExecFnNoArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecEcall => "ecall",
            ExecEbreak => "ebreak",
            ExecUret => "uret",
            ExecSret => "sret",
            ExecMret => "mret",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdFmPredRdRs1Succ {
    ExecFence,
}

impl fmt::Display for ExecFnRdFmPredRdRs1Succ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecFence => "fence",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdImm {
    ExecAuipc,
    ExecLui,
    ExecJal,
}

impl fmt::Display for ExecFnRdImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecAuipc => "auipc",
            ExecLui => "lui",
            ExecJal => "jal",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdRs1 {
    ExecFmvXW,
    ExecFmvWX,
    ExecFclassS,
}

impl fmt::Display for ExecFnRdRs1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecFmvXW => "fmv.x.w",
            ExecFmvWX => "fmv.w.x",
            ExecFclassS => "fclass.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdRs1Imm {
    ExecLb,
    ExecLh,
    ExecLw,
    ExecLbu,
    ExecLhu,
    ExecFlw,
    ExecFenceI,
    ExecAddi,
    ExecSlli,
    ExecSlti,
    ExecSltiu,
    ExecXori,
    ExecSrli,
    ExecSrai,
    ExecOri,
    ExecAndi,
    ExecJalr,
}

impl fmt::Display for ExecFnRdRs1Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecLb => "lb",
            ExecLh => "lh",
            ExecLw => "lw",
            ExecLbu => "lbu",
            ExecLhu => "lhu",
            ExecFlw => "flw",
            ExecFenceI => "fence.i",
            ExecAddi => "addi",
            ExecSlli => "slli",
            ExecSlti => "slti",
            ExecSltiu => "sltiu",
            ExecXori => "xori",
            ExecSrli => "srli",
            ExecSrai => "srai",
            ExecOri => "ori",
            ExecAndi => "andi",
            ExecJalr => "jalr",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdRs1Rm {
    ExecFsqrtS,
    ExecFcvtWS,
    ExecFcvtWuS,
    ExecFcvtSW,
    ExecFcvtSWu,
}

impl fmt::Display for ExecFnRdRs1Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecFsqrtS => "fsqrt.s",
            ExecFcvtWS => "fvct.w.s",
            ExecFcvtWuS => "fcvt.wu.s",
            ExecFcvtSW => "fcvt.s.w",
            ExecFcvtSWu => "fvct.s.wu",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdRs1Rs2 {
    ExecAdd,
    ExecMul,
    ExecSub,
    ExecSll,
    ExecMulh,
    ExecSlt,
    ExecMulhsu,
    ExecSltu,
    ExecMulhu,
    ExecXor,
    ExecDiv,
    ExecSrl,
    ExecDivu,
    ExecSra,
    ExecOr,
    ExecRem,
    ExecAnd,
    ExecRemu,
    ExecFsgnjS,
    ExecFminS,
    ExecFleS,
    ExecFsgnjnS,
    ExecFmaxS,
    ExecFltS,
    ExecFsgnjxS,
    ExecFeqS,
}

impl fmt::Display for ExecFnRdRs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecAdd => "add",
            ExecMul => "mul",
            ExecSub => "sub",
            ExecSll => "sll",
            ExecMulh => "mulh",
            ExecSlt => "slt",
            ExecMulhsu => "mulhsu",
            ExecSltu => "sltu",
            ExecMulhu => "mulhu",
            ExecXor => "xor",
            ExecDiv => "div",
            ExecSrl => "srl",
            ExecDivu => "divu",
            ExecSra => "sra",
            ExecOr => "or",
            ExecRem => "rem",
            ExecAnd => "and",
            ExecRemu => "remu",
            ExecFsgnjS => "fsgnjs",
            ExecFminS => "fmins",
            ExecFleS => "fles",
            ExecFsgnjnS => "fsgnjns",
            ExecFmaxS => "fmaxs",
            ExecFltS => "flts",
            ExecFsgnjxS => "fsgnjxs",
            ExecFeqS => "feqs",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRs1Rs2Imm {
    ExecSb,
    ExecSh,
    ExecSw,
    ExecFsw,
    ExecBeq,
    ExecBne,
    ExecBlt,
    ExecBge,
    ExecBltu,
    ExecBgeu,
}

impl fmt::Display for ExecFnRs1Rs2Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecSb => "sb",
            ExecSh => "sh",
            ExecSw => "sw",
            ExecFsw => "fsw",
            ExecBeq => "beq",
            ExecBne => "bne",
            ExecBlt => "blt",
            ExecBge => "bge",
            ExecBltu => "bltu",
            ExecBgeu => "bgeu",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdRs1Rs2Rm {
    ExecFaddS,
    ExecFsubS,
    ExecFmulS,
    ExecFdivS,
}

impl fmt::Display for ExecFnRdRs1Rs2Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecFaddS => "fadd.s",
            ExecFsubS => "fsub.s",
            ExecFmulS => "fmul.s",
            ExecFdivS => "fdiv.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnRdRs1Rs2Rs3Rm {
    ExecFmaddS,
    ExecFmsubS,
    ExecFnmsubS,
    ExecFnmaddS,
}

impl fmt::Display for ExecFnRdRs1Rs2Rs3Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecFmaddS => "fmadd.s",
            ExecFmsubS => "fmsub.s",
            ExecFnmsubS => "fnmsub.s",
            ExecFnmaddS => "fnmadd.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum ExecFnTrap {
    ExecIllegalInstruction,
}

impl fmt::Display for ExecFnTrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ill")
    }
}

use ExecFnCacheLineIndex::*;
use ExecFnNoArgs::*;
use ExecFnRdFmPredRdRs1Succ::*;
use ExecFnRdImm::*;
use ExecFnRdRs1::*;
use ExecFnRdRs1Imm::*;
use ExecFnRdRs1Rm::*;
use ExecFnRdRs1Rs2::*;
use ExecFnRdRs1Rs2Rm::*;
use ExecFnRdRs1Rs2Rs3Rm::*;
use ExecFnRs1Rs2Imm::*;
use ExecFnTrap::*;

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
    RdImm {
        opcode: ExecFnRdImm, // Which opcodes are viable for these parameters.
        rd: u8,              // Destination register.
        imm: i32,            // Immediate operand.
    },
    RdRs1 {
        opcode: ExecFnRdRs1, // Which opcodes are viable for these parameters.
        rd: u8,              // Destination register.
        rs1: u8,             // Source register.
    },
    RdRs1Imm {
        opcode: ExecFnRdRs1Imm, // Which opcodes are viable for these parameters.
        rd: u8,                 // Destination register.
        rs1: u8,                // Source register.
        imm: i32,               // Immediate operand.
    },
    RdRs1Rs2 {
        opcode: ExecFnRdRs1Rs2, // Which opcodes are viable for these parameters.
        rd: u8,                 // Destination register.
        rs1: u8,                // First source register.
        rs2: u8,                // Second source register.
    },
    Rs1Rs2Imm {
        opcode: ExecFnRs1Rs2Imm, // Which opcodes are viable for these parameters.
        rs1: u8,                 // First source register.
        rs2: u8,                 // Second source register.
        imm: i32,                // Immediate operand.
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

trait Decoder {
    type Item;

    fn gen_trap(&mut self, opcode: ExecFnTrap, ins: u32) -> Self::Item;
    fn gen_no_args(&mut self, opcode: ExecFnNoArgs, ins: u32) -> Self::Item;
    fn gen_jimm20_rd(&mut self, opcode: ExecFnRdImm, ins: u32) -> Self::Item;
    fn gen_bimm12hi_bimm12lo_rs1_rs2(&mut self, opcode: ExecFnRs1Rs2Imm, ins: u32) -> Self::Item;
    fn gen_rd_rm_rs1(&mut self, opcode: ExecFnRdRs1Rm, ins: u32) -> Self::Item;
    fn gen_rd_rm_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2Rm, ins: u32) -> Self::Item;
    fn gen_rd_rs1(&mut self, opcode: ExecFnRdRs1, ins: u32) -> Self::Item;
    fn gen_rd_rm_rs1_rs2_rs3(&mut self, opcode: ExecFnRdRs1Rs2Rs3Rm, ins: u32) -> Self::Item;
    fn gen_rd_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2, ins: u32) -> Self::Item;
    fn gen_imm12hi_imm12lo_rs1_rs2(&mut self, opcode: ExecFnRs1Rs2Imm, ins: u32) -> Self::Item;
    fn gen_imm20_rd(&mut self, opcode: ExecFnRdImm, ins: u32) -> Self::Item;
    fn gen_rd_rs1_shamtw(&mut self, opcode: ExecFnRdRs1Imm, ins: u32) -> Self::Item;
    fn gen_fm_pred_rd_rs1_succ(&mut self, opcode: ExecFnRdFmPredRdRs1Succ, ins: u32) -> Self::Item;
    fn gen_imm12_rd_rs1(&mut self, opcode: ExecFnRdRs1Imm, ins: u32) -> Self::Item;
}

struct Generator;

impl Decoder for Generator {
    type Item = DecodedInstruction;

    fn gen_trap(&mut self, opcode: ExecFnTrap, ins: u32) -> DecodedInstruction {
        DecodedInstruction::Ins { opcode, ins }
    }

    fn gen_no_args(&mut self, opcode: ExecFnNoArgs, _ins: u32) -> DecodedInstruction {
        DecodedInstruction::NoArgs { opcode }
    }

    fn gen_jimm20_rd(&mut self, opcode: ExecFnRdImm, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdImm {
            opcode: opcode,
            rd: extract_rd(ins),
            imm: extract_jimmediate(ins),
        }
    }

    fn gen_bimm12hi_bimm12lo_rs1_rs2(
        &mut self,
        opcode: ExecFnRs1Rs2Imm,
        ins: u32,
    ) -> DecodedInstruction {
        DecodedInstruction::Rs1Rs2Imm {
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
        opcode: ExecFnRs1Rs2Imm,
        ins: u32,
    ) -> DecodedInstruction {
        DecodedInstruction::Rs1Rs2Imm {
            opcode,
            rs1: extract_rs1(ins),
            rs2: extract_rs2(ins),
            imm: extract_simmediate(ins),
        }
    }

    fn gen_imm20_rd(&mut self, opcode: ExecFnRdImm, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdImm {
            opcode,
            rd: extract_rd(ins),
            imm: extract_uimmediate(ins),
        }
    }

    fn gen_rd_rs1_shamtw(&mut self, opcode: ExecFnRdRs1Imm, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Imm {
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

    fn gen_imm12_rd_rs1(&mut self, opcode: ExecFnRdRs1Imm, ins: u32) -> DecodedInstruction {
        DecodedInstruction::RdRs1Imm {
            opcode,
            rd: extract_rd(ins),
            rs1: extract_rs1(ins),
            imm: extract_iimmediate(ins),
        }
    }
}

struct Disassembler;

impl Decoder for Disassembler {
    type Item = String;

    fn gen_trap(&mut self, opcode: ExecFnTrap, ins: u32) -> Self::Item {
        format!("{}\t{}", opcode, ins)
    }

    fn gen_no_args(&mut self, opcode: ExecFnNoArgs, _ins: u32) -> Self::Item {
        format!("{}", opcode)
    }

    fn gen_jimm20_rd(&mut self, opcode: ExecFnRdImm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}",
            opcode,
            extract_rd(ins),
            extract_jimmediate(ins)
        )
    }

    fn gen_bimm12hi_bimm12lo_rs1_rs2(&mut self, opcode: ExecFnRs1Rs2Imm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_rs1(ins),
            extract_rs2(ins),
            extract_bimmediate(ins)
        )
    }

    fn gen_rd_rm_rs1(&mut self, opcode: ExecFnRdRs1Rm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_rd(ins),
            extract_rs1(ins),
            extract_rm(ins)
        )
    }

    fn gen_rd_rm_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2Rm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}, {}",
            opcode,
            extract_rd(ins),
            extract_rs1(ins),
            extract_rs2(ins),
            extract_rm(ins)
        )
    }

    fn gen_rd_rs1(&mut self, opcode: ExecFnRdRs1, ins: u32) -> Self::Item {
        format!("{}\t{}, {}", opcode, extract_rd(ins), extract_rs1(ins))
    }

    fn gen_rd_rm_rs1_rs2_rs3(&mut self, opcode: ExecFnRdRs1Rs2Rs3Rm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}, {}, {}",
            opcode,
            extract_rd(ins),
            extract_rs1(ins),
            extract_rs2(ins),
            extract_rs3(ins),
            extract_rm(ins)
        )
    }

    fn gen_rd_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_rd(ins),
            extract_rs1(ins),
            extract_rs2(ins)
        )
    }

    fn gen_imm12hi_imm12lo_rs1_rs2(&mut self, opcode: ExecFnRs1Rs2Imm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_rs1(ins),
            extract_rs2(ins),
            extract_simmediate(ins)
        )
    }

    fn gen_imm20_rd(&mut self, opcode: ExecFnRdImm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}",
            opcode,
            extract_rd(ins),
            extract_uimmediate(ins) >> 12 // TODO: Does the shift belong here, or with extract_uimmediate()?
        )
    }

    fn gen_rd_rs1_shamtw(&mut self, opcode: ExecFnRdRs1Imm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_rd(ins),
            extract_rs1(ins),
            extract_iimmediate(ins)
        )
    }

    fn gen_fm_pred_rd_rs1_succ(&mut self, opcode: ExecFnRdFmPredRdRs1Succ, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_fm(ins),
            extract_rd(ins),
            extract_rs1(ins)
        )
    }

    fn gen_imm12_rd_rs1(&mut self, opcode: ExecFnRdRs1Imm, ins: u32) -> Self::Item {
        format!(
            "{}\t{}, {}, {}",
            opcode,
            extract_rd(ins),
            extract_rs1(ins),
            extract_iimmediate(ins)
        )
    }
}

// fn decode<T: Decoder + Decoder<Item=U>, U>(decoder: &mut T, ins: u32) -> U {
// fn decode<T, U>(decoder: &mut T, ins: u32) -> U
// where
//     T: Decoder + Decoder<Item = U>,
// {
fn decode<T>(decoder: &mut (impl Decoder + Decoder<Item = T>), ins: u32) -> T {
    // This function is generated by make_decoder.py. Do not edit.
    match bits(ins, 1, 0) {
        0x3 => {
            match bits(ins, 6, 2) {
                0x0 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.gen_imm12_rd_rs1(ExecLb, ins), // lb
                        0x1 => return decoder.gen_imm12_rd_rs1(ExecLh, ins), // lh
                        0x2 => return decoder.gen_imm12_rd_rs1(ExecLw, ins), // lw
                        0x4 => return decoder.gen_imm12_rd_rs1(ExecLbu, ins), // lbu
                        0x5 => return decoder.gen_imm12_rd_rs1(ExecLhu, ins), // lhu
                        _ => {}
                    }
                }
                0x1 => {
                    match bits(ins, 14, 12) {
                        0x2 => return decoder.gen_imm12_rd_rs1(ExecFlw, ins), // flw
                        _ => {}
                    }
                }
                0x3 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.gen_fm_pred_rd_rs1_succ(ExecFence, ins), // fence
                        0x1 => return decoder.gen_imm12_rd_rs1(ExecFenceI, ins),       // fence.i
                        _ => {}
                    }
                }
                0x4 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.gen_imm12_rd_rs1(ExecAddi, ins), // addi
                        0x1 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_shamtw(ExecSlli, ins), // slli
                                _ => {}
                            }
                        }
                        0x2 => return decoder.gen_imm12_rd_rs1(ExecSlti, ins), // slti
                        0x3 => return decoder.gen_imm12_rd_rs1(ExecSltiu, ins), // sltiu
                        0x4 => return decoder.gen_imm12_rd_rs1(ExecXori, ins), // xori
                        0x5 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_shamtw(ExecSrli, ins), // srli
                                0x20 => return decoder.gen_rd_rs1_shamtw(ExecSrai, ins), // srai
                                _ => {}
                            }
                        }
                        0x6 => return decoder.gen_imm12_rd_rs1(ExecOri, ins), // ori
                        0x7 => return decoder.gen_imm12_rd_rs1(ExecAndi, ins), // andi
                        _ => {}
                    }
                }
                0x5 => return decoder.gen_imm20_rd(ExecAuipc, ins), // auipc
                0x8 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.gen_imm12hi_imm12lo_rs1_rs2(ExecSb, ins), // sb
                        0x1 => return decoder.gen_imm12hi_imm12lo_rs1_rs2(ExecSh, ins), // sh
                        0x2 => return decoder.gen_imm12hi_imm12lo_rs1_rs2(ExecSw, ins), // sw
                        _ => {}
                    }
                }
                0x9 => {
                    match bits(ins, 14, 12) {
                        0x2 => return decoder.gen_imm12hi_imm12lo_rs1_rs2(ExecFsw, ins), // fsw
                        _ => {}
                    }
                }
                0xc => {
                    match bits(ins, 14, 12) {
                        0x0 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecAdd, ins), // add
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecMul, ins), // mul
                                0x20 => return decoder.gen_rd_rs1_rs2(ExecSub, ins), // sub
                                _ => {}
                            }
                        }
                        0x1 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecSll, ins), // sll
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecMulh, ins), // mulh
                                _ => {}
                            }
                        }
                        0x2 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecSlt, ins), // slt
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecMulhsu, ins), // mulhsu
                                _ => {}
                            }
                        }
                        0x3 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecSltu, ins), // sltu
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecMulhu, ins), // mulhu
                                _ => {}
                            }
                        }
                        0x4 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecXor, ins), // xor
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecDiv, ins), // div
                                _ => {}
                            }
                        }
                        0x5 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecSrl, ins), // srl
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecDivu, ins), // divu
                                0x20 => return decoder.gen_rd_rs1_rs2(ExecSra, ins), // sra
                                _ => {}
                            }
                        }
                        0x6 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecOr, ins), // or
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecRem, ins), // rem
                                _ => {}
                            }
                        }
                        0x7 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.gen_rd_rs1_rs2(ExecAnd, ins), // and
                                0x1 => return decoder.gen_rd_rs1_rs2(ExecRemu, ins), // remu
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                0xd => return decoder.gen_imm20_rd(ExecLui, ins), // lui
                0x10 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.gen_rd_rm_rs1_rs2_rs3(ExecFmaddS, ins), // fmadd.s
                        _ => {}
                    }
                }
                0x11 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.gen_rd_rm_rs1_rs2_rs3(ExecFmsubS, ins), // fmsub.s
                        _ => {}
                    }
                }
                0x12 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.gen_rd_rm_rs1_rs2_rs3(ExecFnmsubS, ins), // fnmsub.s
                        _ => {}
                    }
                }
                0x13 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.gen_rd_rm_rs1_rs2_rs3(ExecFnmaddS, ins), // fnmadd.s
                        _ => {}
                    }
                }
                0x14 => {
                    match bits(ins, 26, 25) {
                        0x0 => {
                            match bits(ins, 14, 12) {
                                0x0 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.gen_rd_rs1_rs2(ExecFsgnjS, ins), // fsgnj.s
                                        0x5 => return decoder.gen_rd_rs1_rs2(ExecFminS, ins), // fmin.s
                                        0x14 => return decoder.gen_rd_rs1_rs2(ExecFleS, ins), // fle.s
                                        0x1c => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.gen_rd_rs1(ExecFmvXW, ins), // fmv.x.w
                                                _ => {}
                                            }
                                        }
                                        0x1e => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.gen_rd_rs1(ExecFmvWX, ins), // fmv.w.x
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x1 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.gen_rd_rs1_rs2(ExecFsgnjnS, ins), // fsgnjn.s
                                        0x5 => return decoder.gen_rd_rs1_rs2(ExecFmaxS, ins), // fmax.s
                                        0x14 => return decoder.gen_rd_rs1_rs2(ExecFltS, ins), // flt.s
                                        0x1c => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.gen_rd_rs1(ExecFclassS, ins), // fclass.s
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x2 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.gen_rd_rs1_rs2(ExecFsgnjxS, ins), // fsgnjx.s
                                        0x14 => return decoder.gen_rd_rs1_rs2(ExecFeqS, ins), // feq.s
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match bits(ins, 31, 27) {
                                0x0 => return decoder.gen_rd_rm_rs1_rs2(ExecFaddS, ins), // fadd.s
                                0x1 => return decoder.gen_rd_rm_rs1_rs2(ExecFsubS, ins), // fsub.s
                                0x2 => return decoder.gen_rd_rm_rs1_rs2(ExecFmulS, ins), // fmul.s
                                0x3 => return decoder.gen_rd_rm_rs1_rs2(ExecFdivS, ins), // fdiv.s
                                0xb => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.gen_rd_rm_rs1(ExecFsqrtS, ins), // fsqrt.s
                                        _ => {}
                                    }
                                }
                                0x18 => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.gen_rd_rm_rs1(ExecFcvtWS, ins), // fcvt.w.s
                                        0x1 => return decoder.gen_rd_rm_rs1(ExecFcvtWuS, ins), // fcvt.wu.s
                                        _ => {}
                                    }
                                }
                                0x1a => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.gen_rd_rm_rs1(ExecFcvtSW, ins), // fcvt.s.w
                                        0x1 => return decoder.gen_rd_rm_rs1(ExecFcvtSWu, ins), // fcvt.s.wu
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                0x18 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.gen_bimm12hi_bimm12lo_rs1_rs2(ExecBeq, ins), // beq
                        0x1 => return decoder.gen_bimm12hi_bimm12lo_rs1_rs2(ExecBne, ins), // bne
                        0x4 => return decoder.gen_bimm12hi_bimm12lo_rs1_rs2(ExecBlt, ins), // blt
                        0x5 => return decoder.gen_bimm12hi_bimm12lo_rs1_rs2(ExecBge, ins), // bge
                        0x6 => return decoder.gen_bimm12hi_bimm12lo_rs1_rs2(ExecBltu, ins), // bltu
                        0x7 => return decoder.gen_bimm12hi_bimm12lo_rs1_rs2(ExecBgeu, ins), // bgeu
                        _ => {}
                    }
                }
                0x19 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.gen_imm12_rd_rs1(ExecJalr, ins), // jalr
                        _ => {}
                    }
                }
                0x1b => return decoder.gen_jimm20_rd(ExecJal, ins), // jal
                0x1c => {
                    match bits(ins, 14, 12) {
                        0x0 => {
                            match bits(ins, 31, 20) {
                                0x0 => {
                                    match bits(ins, 19, 15) {
                                        0x0 => {
                                            match bits(ins, 11, 7) {
                                                0x0 => return decoder.gen_no_args(ExecEcall, ins), // ecall
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x1 => {
                                    match bits(ins, 19, 15) {
                                        0x0 => {
                                            match bits(ins, 11, 7) {
                                                0x0 => return decoder.gen_no_args(ExecEbreak, ins), // ebreak
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x102 => {
                                    match bits(ins, 19, 15) {
                                        0x0 => {
                                            match bits(ins, 11, 7) {
                                                0x0 => return decoder.gen_no_args(ExecSret, ins), // sret
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x302 => {
                                    match bits(ins, 19, 15) {
                                        0x0 => {
                                            match bits(ins, 11, 7) {
                                                0x0 => return decoder.gen_no_args(ExecMret, ins), // mret
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    // Illegal instruction.
    return decoder.gen_trap(ExecIllegalInstruction, ins);
}
