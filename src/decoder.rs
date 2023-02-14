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

    fn trap(&mut self, instruction: Trap, machine_code: u32) -> Self::Item;
    fn no_args(&mut self, instruction: NoArgs, machine_code: u32) -> Self::Item;
    fn jimm20_rd(&mut self, instruction: Jimm20Rd, machine_code: u32) -> Self::Item;
    fn bimm12hi_bimm12lo_rs1_rs2(
        &mut self,
        instruction: Bimm12Rs1Rs2,
        machine_code: u32,
    ) -> Self::Item;
    fn rd_rm_rs1(&mut self, instruction: RdRs1Rm, machine_code: u32) -> Self::Item;
    fn rd_rm_rs1_rs2(&mut self, instruction: RdRs1Rs2Rm, machine_code: u32) -> Self::Item;
    fn rd_rs1(&mut self, instruction: RdRs1, machine_code: u32) -> Self::Item;
    fn rd_rm_rs1_rs2_rs3(&mut self, instruction: RdRs1Rs2Rs3Rm, machine_code: u32) -> Self::Item;
    fn rd_rs1_rs2(&mut self, instruction: RdRs1Rs2, machine_code: u32) -> Self::Item;
    fn imm12hi_imm12lo_rs1_rs2(
        &mut self,
        instruction: Imm12Rs1Rs2,
        machine_code: u32,
    ) -> Self::Item;
    fn imm20_rd(&mut self, instruction: Imm20Rd, machine_code: u32) -> Self::Item;
    fn rd_rs1_shamtw(&mut self, instruction: RdRs1Shamtw, machine_code: u32) -> Self::Item;
    fn fm_pred_rd_rs1_succ(
        &mut self,
        instruction: RdFmPredRdRs1Succ,
        machine_code: u32,
    ) -> Self::Item;
    fn imm12_rd_rs1(&mut self, instruction: Imm12RdRs1, machine_code: u32) -> Self::Item;
}

fn bits(n: u32, hi: u32, lo: u32) -> u32 {
    let run = (hi - lo) + 1;
    let mask = ((1 << run) - 1) << lo;
    (n & mask) >> lo
}

pub fn decode<T>(decoder: &mut (impl Decoder + Decoder<Item = T>), machine_code: u32) -> T {
    // This function is generated by make_decoder.py. Do not edit.
    match bits(machine_code, 1, 0) {
        0x3 => {
            match bits(machine_code, 6, 2) {
                0x0 => {
                    match bits(machine_code, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(Lb, machine_code), // lb
                        0x1 => return decoder.imm12_rd_rs1(Lh, machine_code), // lh
                        0x2 => return decoder.imm12_rd_rs1(Lw, machine_code), // lw
                        0x4 => return decoder.imm12_rd_rs1(Lbu, machine_code), // lbu
                        0x5 => return decoder.imm12_rd_rs1(Lhu, machine_code), // lhu
                        _ => {}
                    }
                }
                0x1 => {
                    match bits(machine_code, 14, 12) {
                        0x2 => return decoder.imm12_rd_rs1(Flw, machine_code), // flw
                        _ => {}
                    }
                }
                0x3 => {
                    match bits(machine_code, 14, 12) {
                        0x0 => return decoder.fm_pred_rd_rs1_succ(Fence, machine_code), // fence
                        0x1 => return decoder.imm12_rd_rs1(FenceI, machine_code),       // fence.i
                        _ => {}
                    }
                }
                0x4 => {
                    match bits(machine_code, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(Addi, machine_code), // addi
                        0x1 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_shamtw(Slli, machine_code), // slli
                                _ => {}
                            }
                        }
                        0x2 => return decoder.imm12_rd_rs1(Slti, machine_code), // slti
                        0x3 => return decoder.imm12_rd_rs1(Sltiu, machine_code), // sltiu
                        0x4 => return decoder.imm12_rd_rs1(Xori, machine_code), // xori
                        0x5 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_shamtw(Srli, machine_code), // srli
                                0x20 => return decoder.rd_rs1_shamtw(Srai, machine_code), // srai
                                _ => {}
                            }
                        }
                        0x6 => return decoder.imm12_rd_rs1(Ori, machine_code), // ori
                        0x7 => return decoder.imm12_rd_rs1(Andi, machine_code), // andi
                        _ => {}
                    }
                }
                0x5 => return decoder.imm20_rd(Auipc, machine_code), // auipc
                0x8 => {
                    match bits(machine_code, 14, 12) {
                        0x0 => return decoder.imm12hi_imm12lo_rs1_rs2(Sb, machine_code), // sb
                        0x1 => return decoder.imm12hi_imm12lo_rs1_rs2(Sh, machine_code), // sh
                        0x2 => return decoder.imm12hi_imm12lo_rs1_rs2(Sw, machine_code), // sw
                        _ => {}
                    }
                }
                0x9 => {
                    match bits(machine_code, 14, 12) {
                        0x2 => return decoder.imm12hi_imm12lo_rs1_rs2(Fsw, machine_code), // fsw
                        _ => {}
                    }
                }
                0xc => {
                    match bits(machine_code, 14, 12) {
                        0x0 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Add, machine_code), // add
                                0x1 => return decoder.rd_rs1_rs2(Mul, machine_code), // mul
                                0x20 => return decoder.rd_rs1_rs2(Sub, machine_code), // sub
                                _ => {}
                            }
                        }
                        0x1 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Sll, machine_code), // sll
                                0x1 => return decoder.rd_rs1_rs2(Mulh, machine_code), // mulh
                                _ => {}
                            }
                        }
                        0x2 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Slt, machine_code), // slt
                                0x1 => return decoder.rd_rs1_rs2(Mulhsu, machine_code), // mulhsu
                                _ => {}
                            }
                        }
                        0x3 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Sltu, machine_code), // sltu
                                0x1 => return decoder.rd_rs1_rs2(Mulhu, machine_code), // mulhu
                                _ => {}
                            }
                        }
                        0x4 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Xor, machine_code), // xor
                                0x1 => return decoder.rd_rs1_rs2(Div, machine_code), // div
                                _ => {}
                            }
                        }
                        0x5 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Srl, machine_code), // srl
                                0x1 => return decoder.rd_rs1_rs2(Divu, machine_code), // divu
                                0x20 => return decoder.rd_rs1_rs2(Sra, machine_code), // sra
                                _ => {}
                            }
                        }
                        0x6 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(Or, machine_code), // or
                                0x1 => return decoder.rd_rs1_rs2(Rem, machine_code), // rem
                                _ => {}
                            }
                        }
                        0x7 => {
                            match bits(machine_code, 31, 25) {
                                0x0 => return decoder.rd_rs1_rs2(And, machine_code), // and
                                0x1 => return decoder.rd_rs1_rs2(Remu, machine_code), // remu
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                0xd => return decoder.imm20_rd(Lui, machine_code), // lui
                0x10 => {
                    match bits(machine_code, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FmaddS, machine_code), // fmadd.s
                        _ => {}
                    }
                }
                0x11 => {
                    match bits(machine_code, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FmsubS, machine_code), // fmsub.s
                        _ => {}
                    }
                }
                0x12 => {
                    match bits(machine_code, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FnmsubS, machine_code), // fnmsub.s
                        _ => {}
                    }
                }
                0x13 => {
                    match bits(machine_code, 26, 25) {
                        0x0 => return decoder.rd_rm_rs1_rs2_rs3(FnmaddS, machine_code), // fnmadd.s
                        _ => {}
                    }
                }
                0x14 => {
                    match bits(machine_code, 26, 25) {
                        0x0 => {
                            match bits(machine_code, 14, 12) {
                                0x0 => {
                                    match bits(machine_code, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(FsgnjS, machine_code), // fsgnj.s
                                        0x5 => return decoder.rd_rs1_rs2(FminS, machine_code), // fmin.s
                                        0x14 => return decoder.rd_rs1_rs2(FleS, machine_code), // fle.s
                                        0x1c => {
                                            match bits(machine_code, 24, 20) {
                                                0x0 => return decoder.rd_rs1(FmvXW, machine_code), // fmv.x.w
                                                _ => {}
                                            }
                                        }
                                        0x1e => {
                                            match bits(machine_code, 24, 20) {
                                                0x0 => return decoder.rd_rs1(FmvWX, machine_code), // fmv.w.x
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x1 => {
                                    match bits(machine_code, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(FsgnjnS, machine_code), // fsgnjn.s
                                        0x5 => return decoder.rd_rs1_rs2(FmaxS, machine_code), // fmax.s
                                        0x14 => return decoder.rd_rs1_rs2(FltS, machine_code), // flt.s
                                        0x1c => {
                                            match bits(machine_code, 24, 20) {
                                                0x0 => {
                                                    return decoder.rd_rs1(FclassS, machine_code)
                                                } // fclass.s
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x2 => {
                                    match bits(machine_code, 31, 27) {
                                        0x4 => return decoder.rd_rs1_rs2(FsgnjxS, machine_code), // fsgnjx.s
                                        0x14 => return decoder.rd_rs1_rs2(FeqS, machine_code), // feq.s
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match bits(machine_code, 31, 27) {
                                0x0 => return decoder.rd_rm_rs1_rs2(FaddS, machine_code), // fadd.s
                                0x1 => return decoder.rd_rm_rs1_rs2(FsubS, machine_code), // fsub.s
                                0x2 => return decoder.rd_rm_rs1_rs2(FmulS, machine_code), // fmul.s
                                0x3 => return decoder.rd_rm_rs1_rs2(FdivS, machine_code), // fdiv.s
                                0xb => {
                                    match bits(machine_code, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(FsqrtS, machine_code), // fsqrt.s
                                        _ => {}
                                    }
                                }
                                0x18 => {
                                    match bits(machine_code, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(FcvtWS, machine_code), // fcvt.w.s
                                        0x1 => return decoder.rd_rm_rs1(FcvtWuS, machine_code), // fcvt.wu.s
                                        _ => {}
                                    }
                                }
                                0x1a => {
                                    match bits(machine_code, 24, 20) {
                                        0x0 => return decoder.rd_rm_rs1(FcvtSW, machine_code), // fcvt.s.w
                                        0x1 => return decoder.rd_rm_rs1(FcvtSWu, machine_code), // fcvt.s.wu
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
                    match bits(machine_code, 14, 12) {
                        0x0 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Beq, machine_code), // beq
                        0x1 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bne, machine_code), // bne
                        0x4 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Blt, machine_code), // blt
                        0x5 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bge, machine_code), // bge
                        0x6 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bltu, machine_code), // bltu
                        0x7 => return decoder.bimm12hi_bimm12lo_rs1_rs2(Bgeu, machine_code), // bgeu
                        _ => {}
                    }
                }
                0x19 => {
                    match bits(machine_code, 14, 12) {
                        0x0 => return decoder.imm12_rd_rs1(Jalr, machine_code), // jalr
                        _ => {}
                    }
                }
                0x1b => return decoder.jimm20_rd(Jal, machine_code), // jal
                0x1c => {
                    match bits(machine_code, 14, 12) {
                        0x0 => {
                            match bits(machine_code, 31, 20) {
                                0x0 => {
                                    match bits(machine_code, 19, 15) {
                                        0x0 => {
                                            match bits(machine_code, 11, 7) {
                                                0x0 => return decoder.no_args(Ecall, machine_code), // ecall
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x1 => {
                                    match bits(machine_code, 19, 15) {
                                        0x0 => {
                                            match bits(machine_code, 11, 7) {
                                                0x0 => {
                                                    return decoder.no_args(Ebreak, machine_code)
                                                } // ebreak
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x102 => {
                                    match bits(machine_code, 19, 15) {
                                        0x0 => {
                                            match bits(machine_code, 11, 7) {
                                                0x0 => return decoder.no_args(Sret, machine_code), // sret
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                0x302 => {
                                    match bits(machine_code, 19, 15) {
                                        0x0 => {
                                            match bits(machine_code, 11, 7) {
                                                0x0 => return decoder.no_args(Mret, machine_code), // mret
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
    return decoder.trap(IllegalInstruction, machine_code);
}
