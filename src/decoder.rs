use std::fmt;

#[derive(Debug)]
pub enum CacheLineIndex {
    FetchDecodeReplace,
}

#[derive(Debug)]
pub enum NoArgs {
    Ecall,
    Ebreak,
    Uret,
    Sret,
    Mret,
}

impl fmt::Display for NoArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ecall => "ecall",
            Ebreak => "ebreak",
            Uret => "uret",
            Sret => "sret",
            Mret => "mret",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdFmPredRdRs1Succ {
    Fence,
}

impl fmt::Display for RdFmPredRdRs1Succ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Fence => "fence",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Imm20Rd {
    Auipc,
    Lui,
}

impl fmt::Display for Imm20Rd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Auipc => "auipc",
            Lui => "lui",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Jimm20Rd {
    Jal,
}

impl fmt::Display for Jimm20Rd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Jal => "jal",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdRs1 {
    FmvXW,
    FmvWX,
    FclassS,
}

impl fmt::Display for RdRs1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FmvXW => "fmv.x.w",
            FmvWX => "fmv.w.x",
            FclassS => "fclass.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Imm12RdRs1 {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Flw,
    FenceI,
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Jalr,
}

impl fmt::Display for Imm12RdRs1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Lb => "lb",
            Lh => "lh",
            Lw => "lw",
            Lbu => "lbu",
            Lhu => "lhu",
            Flw => "flw",
            FenceI => "fence.i",
            Addi => "addi",
            Slti => "slti",
            Sltiu => "sltiu",
            Xori => "xori",
            Ori => "ori",
            Andi => "andi",
            Jalr => "jalr",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdRs1Shamtw {
    Slli,
    Srli,
    Srai,
}

impl fmt::Display for RdRs1Shamtw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Slli => "slli",
            Srli => "srli",
            Srai => "srai",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdRs1Rm {
    FsqrtS,
    FcvtWS,
    FcvtWuS,
    FcvtSW,
    FcvtSWu,
}

impl fmt::Display for RdRs1Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FsqrtS => "fsqrt.s",
            FcvtWS => "fvct.w.s",
            FcvtWuS => "fcvt.wu.s",
            FcvtSW => "fcvt.s.w",
            FcvtSWu => "fvct.s.wu",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdRs1Rs2 {
    Add,
    Mul,
    Sub,
    Sll,
    Mulh,
    Slt,
    Mulhsu,
    Sltu,
    Mulhu,
    Xor,
    Div,
    Srl,
    Divu,
    Sra,
    Or,
    Rem,
    And,
    Remu,
    FsgnjS,
    FminS,
    FleS,
    FsgnjnS,
    FmaxS,
    FltS,
    FsgnjxS,
    FeqS,
}

impl fmt::Display for RdRs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Add => "add",
            Mul => "mul",
            Sub => "sub",
            Sll => "sll",
            Mulh => "mulh",
            Slt => "slt",
            Mulhsu => "mulhsu",
            Sltu => "sltu",
            Mulhu => "mulhu",
            Xor => "xor",
            Div => "div",
            Srl => "srl",
            Divu => "divu",
            Sra => "sra",
            Or => "or",
            Rem => "rem",
            And => "and",
            Remu => "remu",
            FsgnjS => "fsgnj.s",
            FminS => "fmin.s",
            FleS => "fle.s",
            FsgnjnS => "fsgnjn.s",
            FmaxS => "fmax.s",
            FltS => "flt.s",
            FsgnjxS => "fsgnjx.s",
            FeqS => "feq.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Imm12Rs1Rs2 {
    Sb,
    Sh,
    Sw,
    Fsw,
}

impl fmt::Display for Imm12Rs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Sb => "sb",
            Sh => "sh",
            Sw => "sw",
            Fsw => "fsw",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Bimm12Rs1Rs2 {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl fmt::Display for Bimm12Rs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Beq => "beq",
            Bne => "bne",
            Blt => "blt",
            Bge => "bge",
            Bltu => "bltu",
            Bgeu => "bgeu",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdRs1Rs2Rm {
    FaddS,
    FsubS,
    FmulS,
    FdivS,
}

impl fmt::Display for RdRs1Rs2Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FaddS => "fadd.s",
            FsubS => "fsub.s",
            FmulS => "fmul.s",
            FdivS => "fdiv.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RdRs1Rs2Rs3Rm {
    FmaddS,
    FmsubS,
    FnmsubS,
    FnmaddS,
}

impl fmt::Display for RdRs1Rs2Rs3Rm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FmaddS => "fmadd.s",
            FmsubS => "fmsub.s",
            FnmsubS => "fnmsub.s",
            FnmaddS => "fnmadd.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Trap {
    IllegalInstruction,
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ill")
    }
}

use Bimm12Rs1Rs2::*;
use CacheLineIndex::*;
use Imm12RdRs1::*;
use Imm12Rs1Rs2::*;
use Imm20Rd::*;
use Jimm20Rd::*;
use NoArgs::*;
use RdFmPredRdRs1Succ::*;
use RdRs1::*;
use RdRs1Rm::*;
use RdRs1Rs2::*;
use RdRs1Rs2Rm::*;
use RdRs1Rs2Rs3Rm::*;
use RdRs1Shamtw::*;
use Trap::*;

pub trait Decoder {
    type Item;

    fn trap(&mut self, opcode: Trap, ins: u32) -> Self::Item;
    fn no_args(&mut self, opcode: NoArgs, ins: u32) -> Self::Item;
    fn jimm20_rd(&mut self, opcode: Jimm20Rd, ins: u32) -> Self::Item;
    fn bimm12hi_bimm12lo_rs1_rs2(&mut self, opcode: Bimm12Rs1Rs2, ins: u32) -> Self::Item;
    fn rd_rm_rs1(&mut self, opcode: RdRs1Rm, ins: u32) -> Self::Item;
    fn rd_rm_rs1_rs2(&mut self, opcode: RdRs1Rs2Rm, ins: u32) -> Self::Item;
    fn rd_rs1(&mut self, opcode: RdRs1, ins: u32) -> Self::Item;
    fn rd_rm_rs1_rs2_rs3(&mut self, opcode: RdRs1Rs2Rs3Rm, ins: u32) -> Self::Item;
    fn rd_rs1_rs2(&mut self, opcode: RdRs1Rs2, ins: u32) -> Self::Item;
    fn imm12hi_imm12lo_rs1_rs2(&mut self, opcode: Imm12Rs1Rs2, ins: u32) -> Self::Item;
    fn imm20_rd(&mut self, opcode: Imm20Rd, ins: u32) -> Self::Item;
    fn rd_rs1_shamtw(&mut self, opcode: RdRs1Shamtw, ins: u32) -> Self::Item;
    fn fm_pred_rd_rs1_succ(&mut self, opcode: RdFmPredRdRs1Succ, ins: u32) -> Self::Item;
    fn imm12_rd_rs1(&mut self, opcode: Imm12RdRs1, ins: u32) -> Self::Item;
}

fn bits(n: u32, hi: u32, lo: u32) -> u32 {
    let run = (hi - lo) + 1;
    let mask = ((1 << run) - 1) << lo;
    (n & mask) >> lo
}

pub fn decode<T>(decoder: &mut (impl Decoder + Decoder<Item = T>), ins: u32) -> T {
    // This function is generated by make_decoder.py. Do not edit.
    match bits(ins, 1, 0) {
        0x3 => {
            match bits(ins, 6, 2) {
                0x0 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(Lb, ins),  // lb
                        0x1 => return decoder.imm12_rd_rs1(Lh, ins),  // lh
                        0x2 => return decoder.imm12_rd_rs1(Lw, ins),  // lw
                        0x4 => return decoder.imm12_rd_rs1(Lbu, ins), // lbu
                        0x5 => return decoder.imm12_rd_rs1(Lhu, ins), // lhu
                        _ => {}
                    }
                }
                0x1 => {
                    match bits(ins, 14, 12) {
                        0x2 => return decoder.imm12_rd_rs1(Flw, ins), // flw
                        _ => {}
                    }
                }
                0x3 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.fm_pred_rd_rs1_succ(Fence, ins), // fence
                        0x1 => return decoder.imm12_rd_rs1(FenceI, ins),       // fence.i
                        _ => {}
                    }
                }
                0x4 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(Addi, ins), // addi
                        0x1 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_shamtw(Slli, ins), // slli
                                _ => {}
                            }
                        }
                        0x2 => return decoder.imm12_rd_rs1(Slti, ins), // slti
                        0x3 => return decoder.imm12_rd_rs1(Sltiu, ins), // sltiu
                        0x4 => return decoder.imm12_rd_rs1(Xori, ins), // xori
                        0x5 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_shamtw(Srli, ins), // srli
                                0x20 => return decoder.rd_rs1_shamtw(Srai, ins), // srai
                                _ => {}
                            }
                        }
                        0x6 => return decoder.imm12_rd_rs1(Ori, ins), // ori
                        0x7 => return decoder.imm12_rd_rs1(Andi, ins), // andi
                        _ => {}
                    }
                }
                0x5 => return decoder.imm20_rd(Auipc, ins), // auipc
                0x8 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12hi_imm12lo_rs1_rs2(Sb, ins), // sb
                        0x1 => return decoder.imm12hi_imm12lo_rs1_rs2(Sh, ins), // sh
                        0x2 => return decoder.imm12hi_imm12lo_rs1_rs2(Sw, ins), // sw
                        _ => {}
                    }
                }
                0x9 => {
                    match bits(ins, 14, 12) {
                        0x2 => return decoder.imm12hi_imm12lo_rs1_rs2(Fsw, ins), // fsw
                        _ => {}
                    }
                }
                0xc => {
                    match bits(ins, 14, 12) {
                        0x0 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Add, ins),  // add
                                0x1 => return decoder.rd_rs1_rs2(Mul, ins),  // mul
                                0x20 => return decoder.rd_rs1_rs2(Sub, ins), // sub
                                _ => {}
                            }
                        }
                        0x1 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Sll, ins),  // sll
                                0x1 => return decoder.rd_rs1_rs2(Mulh, ins), // mulh
                                _ => {}
                            }
                        }
                        0x2 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Slt, ins),    // slt
                                0x1 => return decoder.rd_rs1_rs2(Mulhsu, ins), // mulhsu
                                _ => {}
                            }
                        }
                        0x3 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Sltu, ins),  // sltu
                                0x1 => return decoder.rd_rs1_rs2(Mulhu, ins), // mulhu
                                _ => {}
                            }
                        }
                        0x4 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Xor, ins), // xor
                                0x1 => return decoder.rd_rs1_rs2(Div, ins), // div
                                _ => {}
                            }
                        }
                        0x5 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Srl, ins),  // srl
                                0x1 => return decoder.rd_rs1_rs2(Divu, ins), // divu
                                0x20 => return decoder.rd_rs1_rs2(Sra, ins), // sra
                                _ => {}
                            }
                        }
                        0x6 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Or, ins),  // or
                                0x1 => return decoder.rd_rs1_rs2(Rem, ins), // rem
                                _ => {}
                            }
                        }
                        0x7 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(And, ins),  // and
                                0x1 => return decoder.rd_rs1_rs2(Remu, ins), // remu
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                0xd => return decoder.imm20_rd(Lui, ins), // lui
                0x10 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FmaddS, ins), // fmadd.s
                        _ => {}
                    }
                }
                0x11 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FmsubS, ins), // fmsub.s
                        _ => {}
                    }
                }
                0x12 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FnmsubS, ins), // fnmsub.s
                        _ => {}
                    }
                }
                0x13 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FnmaddS, ins), // fnmadd.s
                        _ => {}
                    }
                }
                0x14 => {
                    match bits(ins, 26, 25) {
                        0x0 => {
                            match bits(ins, 14, 12) {
                                0x0 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(FsgnjS, ins), // fsgnj.s
                                        0x5 => return decoder.rd_rs1_rs2(FminS, ins),  // fmin.s
                                        0x14 => return decoder.rd_rs1_rs2(FleS, ins),  // fle.s
                                        0x1c => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.rd_rs1(FmvXW, ins), // fmv.x.w
                                                _ => {}
                                            }
                                        }
                                        0x1e => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.rd_rs1(FmvWX, ins), // fmv.w.x
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x1 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(FsgnjnS, ins), // fsgnjn.s
                                        0x5 => return decoder.rd_rs1_rs2(FmaxS, ins),   // fmax.s
                                        0x14 => return decoder.rd_rs1_rs2(FltS, ins),   // flt.s
                                        0x1c => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.rd_rs1(FclassS, ins), // fclass.s
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x2 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(FsgnjxS, ins), // fsgnjx.s
                                        0x14 => return decoder.rd_rs1_rs2(FeqS, ins),   // feq.s
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match bits(ins, 31, 27) {
                                0x0 => return decoder.rd_rm_rs1_rs2(FaddS, ins), // fadd.s
                                0x1 => return decoder.rd_rm_rs1_rs2(FsubS, ins), // fsub.s
                                0x2 => return decoder.rd_rm_rs1_rs2(FmulS, ins), // fmul.s
                                0x3 => return decoder.rd_rm_rs1_rs2(FdivS, ins), // fdiv.s
                                0xb => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(FsqrtS, ins), // fsqrt.s
                                        _ => {}
                                    }
                                }
                                0x18 => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(FcvtWS, ins), // fcvt.w.s
                                        0x1 => return decoder.rd_rm_rs1(FcvtWuS, ins), // fcvt.wu.s
                                        _ => {}
                                    }
                                }
                                0x1a => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(FcvtSW, ins), // fcvt.s.w
                                        0x1 => return decoder.rd_rm_rs1(FcvtSWu, ins), // fcvt.s.wu
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
                        0x0 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Beq, ins), // beq
                        0x1 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bne, ins), // bne
                        0x4 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Blt, ins), // blt
                        0x5 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bge, ins), // bge
                        0x6 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bltu, ins), // bltu
                        0x7 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bgeu, ins), // bgeu
                        _ => {}
                    }
                }
                0x19 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(Jalr, ins), // jalr
                        _ => {}
                    }
                }
                0x1b => return decoder.jimm20_rd(Jal, ins), // jal
                0x1c => {
                    match bits(ins, 14, 12) {
                        0x0 => {
                            match bits(ins, 31, 20) {
                                0x0 => {
                                    match bits(ins, 19, 15) {
                                        0x0 => {
                                            match bits(ins, 11, 7) {
                                                0x0 => return decoder.no_args(Ecall, ins), // ecall
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
                                                0x0 => return decoder.no_args(Ebreak, ins), // ebreak
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
                                                0x0 => return decoder.no_args(Sret, ins), // sret
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
                                                0x0 => return decoder.no_args(Mret, ins), // mret
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
    return decoder.trap(IllegalInstruction, ins);
}
