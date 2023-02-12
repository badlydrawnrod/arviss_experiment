use std::fmt;

#[derive(Debug)]
pub enum ExecFnCacheLineIndex {
    ExecFetchDecodeReplace,
}

#[derive(Debug)]
pub enum ExecFnNoArgs {
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
pub enum ExecFnRdFmPredRdRs1Succ {
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
pub enum ExecFnImm20Rd {
    ExecAuipc,
    ExecLui,
}

impl fmt::Display for ExecFnImm20Rd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecAuipc => "auipc",
            ExecLui => "lui",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ExecFnJimm20Rd {
    ExecJal,
}

impl fmt::Display for ExecFnJimm20Rd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecJal => "jal",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ExecFnRdRs1 {
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
pub enum ExecFnImm12RdRs1 {
    ExecLb,
    ExecLh,
    ExecLw,
    ExecLbu,
    ExecLhu,
    ExecFlw,
    ExecFenceI,
    ExecAddi,
    ExecSlti,
    ExecSltiu,
    ExecXori,
    ExecOri,
    ExecAndi,
    ExecJalr,
}

impl fmt::Display for ExecFnImm12RdRs1 {
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
            ExecSlti => "slti",
            ExecSltiu => "sltiu",
            ExecXori => "xori",
            ExecOri => "ori",
            ExecAndi => "andi",
            ExecJalr => "jalr",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ExecFnRdRs1Shamtw {
    ExecSlli,
    ExecSrli,
    ExecSrai,
}

impl fmt::Display for ExecFnRdRs1Shamtw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecSlli => "slli",
            ExecSrli => "srli",
            ExecSrai => "srai",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ExecFnRdRs1Rm {
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
pub enum ExecFnRdRs1Rs2 {
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
            ExecFsgnjS => "fsgnj.s",
            ExecFminS => "fmin.s",
            ExecFleS => "fle.s",
            ExecFsgnjnS => "fsgnjn.s",
            ExecFmaxS => "fmax.s",
            ExecFltS => "flt.s",
            ExecFsgnjxS => "fsgnjx.s",
            ExecFeqS => "feq.s",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ExecFnImm12Rs1Rs2 {
    ExecSb,
    ExecSh,
    ExecSw,
    ExecFsw,
}

impl fmt::Display for ExecFnImm12Rs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExecSb => "sb",
            ExecSh => "sh",
            ExecSw => "sw",
            ExecFsw => "fsw",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ExecFnBimm12Rs1Rs2 {
    ExecBeq,
    ExecBne,
    ExecBlt,
    ExecBge,
    ExecBltu,
    ExecBgeu,
}

impl fmt::Display for ExecFnBimm12Rs1Rs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
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
pub enum ExecFnRdRs1Rs2Rm {
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
pub enum ExecFnRdRs1Rs2Rs3Rm {
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
pub enum ExecFnTrap {
    ExecIllegalInstruction,
}

impl fmt::Display for ExecFnTrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ill")
    }
}

use ExecFnBimm12Rs1Rs2::*;
use ExecFnCacheLineIndex::*;
use ExecFnImm12RdRs1::*;
use ExecFnImm12Rs1Rs2::*;
use ExecFnImm20Rd::*;
use ExecFnJimm20Rd::*;
use ExecFnNoArgs::*;
use ExecFnRdFmPredRdRs1Succ::*;
use ExecFnRdRs1::*;
use ExecFnRdRs1Rm::*;
use ExecFnRdRs1Rs2::*;
use ExecFnRdRs1Rs2Rm::*;
use ExecFnRdRs1Rs2Rs3Rm::*;
use ExecFnRdRs1Shamtw::*;
use ExecFnTrap::*;

pub trait Decoder {
    type Item;

    fn trap(&mut self, opcode: ExecFnTrap, ins: u32) -> Self::Item;
    fn no_args(&mut self, opcode: ExecFnNoArgs, ins: u32) -> Self::Item;
    fn jimm20_rd(&mut self, opcode: ExecFnJimm20Rd, ins: u32) -> Self::Item;
    fn bimm12hi_bimm12lo_rs1_rs2(&mut self, opcode: ExecFnBimm12Rs1Rs2, ins: u32) -> Self::Item;
    fn rd_rm_rs1(&mut self, opcode: ExecFnRdRs1Rm, ins: u32) -> Self::Item;
    fn rd_rm_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2Rm, ins: u32) -> Self::Item;
    fn rd_rs1(&mut self, opcode: ExecFnRdRs1, ins: u32) -> Self::Item;
    fn rd_rm_rs1_rs2_rs3(&mut self, opcode: ExecFnRdRs1Rs2Rs3Rm, ins: u32) -> Self::Item;
    fn rd_rs1_rs2(&mut self, opcode: ExecFnRdRs1Rs2, ins: u32) -> Self::Item;
    fn imm12hi_imm12lo_rs1_rs2(&mut self, opcode: ExecFnImm12Rs1Rs2, ins: u32) -> Self::Item;
    fn imm20_rd(&mut self, opcode: ExecFnImm20Rd, ins: u32) -> Self::Item;
    fn rd_rs1_shamtw(&mut self, opcode: ExecFnRdRs1Shamtw, ins: u32) -> Self::Item;
    fn fm_pred_rd_rs1_succ(&mut self, opcode: ExecFnRdFmPredRdRs1Succ, ins: u32) -> Self::Item;
    fn imm12_rd_rs1(&mut self, opcode: ExecFnImm12RdRs1, ins: u32) -> Self::Item;
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
                        0x0 => return decoder.imm12_rd_rs1(ExecLb, ins), // lb
                        0x1 => return decoder.imm12_rd_rs1(ExecLh, ins), // lh
                        0x2 => return decoder.imm12_rd_rs1(ExecLw, ins), // lw
                        0x4 => return decoder.imm12_rd_rs1(ExecLbu, ins), // lbu
                        0x5 => return decoder.imm12_rd_rs1(ExecLhu, ins), // lhu
                        _ => {}
                    }
                }
                0x1 => {
                    match bits(ins, 14, 12) {
                        0x2 => return decoder.imm12_rd_rs1(ExecFlw, ins), // flw
                        _ => {}
                    }
                }
                0x3 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.fm_pred_rd_rs1_succ(ExecFence, ins), // fence
                        0x1 => return decoder.imm12_rd_rs1(ExecFenceI, ins),       // fence.i
                        _ => {}
                    }
                }
                0x4 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(ExecAddi, ins), // addi
                        0x1 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_shamtw(ExecSlli, ins), // slli
                                _ => {}
                            }
                        }
                        0x2 => return decoder.imm12_rd_rs1(ExecSlti, ins), // slti
                        0x3 => return decoder.imm12_rd_rs1(ExecSltiu, ins), // sltiu
                        0x4 => return decoder.imm12_rd_rs1(ExecXori, ins), // xori
                        0x5 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_shamtw(ExecSrli, ins), // srli
                                0x20 => return decoder.rd_rs1_shamtw(ExecSrai, ins), // srai
                                _ => {}
                            }
                        }
                        0x6 => return decoder.imm12_rd_rs1(ExecOri, ins), // ori
                        0x7 => return decoder.imm12_rd_rs1(ExecAndi, ins), // andi
                        _ => {}
                    }
                }
                0x5 => return decoder.imm20_rd(ExecAuipc, ins), // auipc
                0x8 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12hi_imm12lo_rs1_rs2(ExecSb, ins), // sb
                        0x1 => return decoder.imm12hi_imm12lo_rs1_rs2(ExecSh, ins), // sh
                        0x2 => return decoder.imm12hi_imm12lo_rs1_rs2(ExecSw, ins), // sw
                        _ => {}
                    }
                }
                0x9 => {
                    match bits(ins, 14, 12) {
                        0x2 => return decoder.imm12hi_imm12lo_rs1_rs2(ExecFsw, ins), // fsw
                        _ => {}
                    }
                }
                0xc => {
                    match bits(ins, 14, 12) {
                        0x0 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecAdd, ins), // add
                                0x1 => return decoder.rd_rs1_rs2(ExecMul, ins), // mul
                                0x20 => return decoder.rd_rs1_rs2(ExecSub, ins), // sub
                                _ => {}
                            }
                        }
                        0x1 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecSll, ins), // sll
                                0x1 => return decoder.rd_rs1_rs2(ExecMulh, ins), // mulh
                                _ => {}
                            }
                        }
                        0x2 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecSlt, ins), // slt
                                0x1 => return decoder.rd_rs1_rs2(ExecMulhsu, ins), // mulhsu
                                _ => {}
                            }
                        }
                        0x3 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecSltu, ins), // sltu
                                0x1 => return decoder.rd_rs1_rs2(ExecMulhu, ins), // mulhu
                                _ => {}
                            }
                        }
                        0x4 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecXor, ins), // xor
                                0x1 => return decoder.rd_rs1_rs2(ExecDiv, ins), // div
                                _ => {}
                            }
                        }
                        0x5 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecSrl, ins), // srl
                                0x1 => return decoder.rd_rs1_rs2(ExecDivu, ins), // divu
                                0x20 => return decoder.rd_rs1_rs2(ExecSra, ins), // sra
                                _ => {}
                            }
                        }
                        0x6 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecOr, ins),  // or
                                0x1 => return decoder.rd_rs1_rs2(ExecRem, ins), // rem
                                _ => {}
                            }
                        }
                        0x7 => {
                            match bits(ins, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(ExecAnd, ins), // and
                                0x1 => return decoder.rd_rs1_rs2(ExecRemu, ins), // remu
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                0xd => return decoder.imm20_rd(ExecLui, ins), // lui
                0x10 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(ExecFmaddS, ins), // fmadd.s
                        _ => {}
                    }
                }
                0x11 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(ExecFmsubS, ins), // fmsub.s
                        _ => {}
                    }
                }
                0x12 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(ExecFnmsubS, ins), // fnmsub.s
                        _ => {}
                    }
                }
                0x13 => {
                    match bits(ins, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(ExecFnmaddS, ins), // fnmadd.s
                        _ => {}
                    }
                }
                0x14 => {
                    match bits(ins, 26, 25) {
                        0x0 => {
                            match bits(ins, 14, 12) {
                                0x0 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(ExecFsgnjS, ins), // fsgnj.s
                                        0x5 => return decoder.rd_rs1_rs2(ExecFminS, ins),  // fmin.s
                                        0x14 => return decoder.rd_rs1_rs2(ExecFleS, ins),  // fle.s
                                        0x1c => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.rd_rs1(ExecFmvXW, ins), // fmv.x.w
                                                _ => {}
                                            }
                                        }
                                        0x1e => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.rd_rs1(ExecFmvWX, ins), // fmv.w.x
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x1 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(ExecFsgnjnS, ins), // fsgnjn.s
                                        0x5 => return decoder.rd_rs1_rs2(ExecFmaxS, ins), // fmax.s
                                        0x14 => return decoder.rd_rs1_rs2(ExecFltS, ins), // flt.s
                                        0x1c => {
                                            match bits(ins, 24, 20) {
                                                0x0 => return decoder.rd_rs1(ExecFclassS, ins), // fclass.s
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x2 => {
                                    match bits(ins, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(ExecFsgnjxS, ins), // fsgnjx.s
                                        0x14 => return decoder.rd_rs1_rs2(ExecFeqS, ins),   // feq.s
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match bits(ins, 31, 27) {
                                0x0 => return decoder.rd_rm_rs1_rs2(ExecFaddS, ins), // fadd.s
                                0x1 => return decoder.rd_rm_rs1_rs2(ExecFsubS, ins), // fsub.s
                                0x2 => return decoder.rd_rm_rs1_rs2(ExecFmulS, ins), // fmul.s
                                0x3 => return decoder.rd_rm_rs1_rs2(ExecFdivS, ins), // fdiv.s
                                0xb => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(ExecFsqrtS, ins), // fsqrt.s
                                        _ => {}
                                    }
                                }
                                0x18 => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(ExecFcvtWS, ins), // fcvt.w.s
                                        0x1 => return decoder.rd_rm_rs1(ExecFcvtWuS, ins), // fcvt.wu.s
                                        _ => {}
                                    }
                                }
                                0x1a => {
                                    match bits(ins, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(ExecFcvtSW, ins), // fcvt.s.w
                                        0x1 => return decoder.rd_rm_rs1(ExecFcvtSWu, ins), // fcvt.s.wu
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
                        0x0 => return decoder.bimm12hi_bimm12lo_rs1_rs2(ExecBeq, ins), // beq
                        0x1 => return decoder.bimm12hi_bimm12lo_rs1_rs2(ExecBne, ins), // bne
                        0x4 => return decoder.bimm12hi_bimm12lo_rs1_rs2(ExecBlt, ins), // blt
                        0x5 => return decoder.bimm12hi_bimm12lo_rs1_rs2(ExecBge, ins), // bge
                        0x6 => return decoder.bimm12hi_bimm12lo_rs1_rs2(ExecBltu, ins), // bltu
                        0x7 => return decoder.bimm12hi_bimm12lo_rs1_rs2(ExecBgeu, ins), // bgeu
                        _ => {}
                    }
                }
                0x19 => {
                    match bits(ins, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(ExecJalr, ins), // jalr
                        _ => {}
                    }
                }
                0x1b => return decoder.jimm20_rd(ExecJal, ins), // jal
                0x1c => {
                    match bits(ins, 14, 12) {
                        0x0 => {
                            match bits(ins, 31, 20) {
                                0x0 => {
                                    match bits(ins, 19, 15) {
                                        0x0 => {
                                            match bits(ins, 11, 7) {
                                                0x0 => return decoder.no_args(ExecEcall, ins), // ecall
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
                                                0x0 => return decoder.no_args(ExecEbreak, ins), // ebreak
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
                                                0x0 => return decoder.no_args(ExecSret, ins), // sret
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
                                                0x0 => return decoder.no_args(ExecMret, ins), // mret
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
    return decoder.trap(ExecIllegalInstruction, ins);
}
