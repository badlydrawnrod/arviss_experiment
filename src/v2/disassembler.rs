use super::{tobits::Reg, DecodeRv32i, DecodeRv32m};

pub struct Disassembler;

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

fn abi(reg: Reg) -> &'static str {
    match reg as usize {
        0..=31 => ABI_NAMES[reg as usize],
        _ => unreachable!(),
    }
}

fn fabi(reg: u32) -> &'static str {
    match reg {
        0..=31 => FABI_NAMES[reg as usize],
        _ => unreachable!(),
    }
}

fn rounding_mode(mode: u32) -> &'static str {
    match mode {
        0..=7 => ROUNDING_MODES[mode as usize],
        _ => unreachable!(),
    }
}

impl DecodeRv32i for Disassembler {
    type Item = String;

    fn illegal(&mut self, ins: u32) -> Self::Item {
        format!("illegal instruction: {:04x}", ins)
    }

    fn beq(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        format!("beq\t{}, {}, {}", abi(rs1), abi(rs2), bimm as i32)
    }

    fn bne(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        format!("bne\t{}, {}, {}", abi(rs1), abi(rs2), bimm as i32)
    }

    fn blt(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        format!("blt\t{}, {}, {}", abi(rs1), abi(rs2), bimm as i32)
    }

    fn bge(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        format!("bge\t{}, {}, {}", abi(rs1), abi(rs2), bimm as i32)
    }

    fn bltu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        format!("bltu\t{}, {}, {}", abi(rs1), abi(rs2), bimm as i32)
    }

    fn bgeu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        format!("bgeu\t{}, {}, {}", abi(rs1), abi(rs2), bimm as i32)
    }

    fn lb(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("lb\t{}, {}({})", abi(rd), iimm as i32, abi(rs1))
    }

    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("lh\t{}, {}({})", abi(rd), iimm as i32, abi(rs1))
    }

    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("lw\t{}, {}({})", abi(rd), iimm as i32, abi(rs1))
    }

    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("lbu\t{}, {}({})", abi(rd), iimm as i32, abi(rs1))
    }

    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("lhu\t{}, {}({})", abi(rd), iimm as i32, abi(rs1))
    }

    fn addi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("addi\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("slti\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("sltiu\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("xori\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("ori\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("andi\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn jalr(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        format!("jalr\t{}, {}, {}", abi(rd), abi(rs1), iimm as i32)
    }

    fn sb(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        format!("sb\t{}, {}({})", abi(rs2), simm as i32, abi(rs1))
    }

    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        format!("sh\t{}, {}({})", abi(rs2), simm as i32, abi(rs1))
    }

    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        format!("sw\t{}, {}({})", abi(rs2), simm as i32, abi(rs1))
    }

    fn auipc(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        format!("auipc\t{}, {}", abi(rd), (uimm as i32) >> 12)
    }

    fn lui(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        format!("lui\t{}, {}", abi(rd), (uimm as i32) >> 12)
    }

    fn jal(&mut self, rd: Reg, jimm: u32) -> Self::Item {
        format!("jal\t{}, {}", abi(rd), jimm as i32)
    }

    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("add\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("sub\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("sll\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("slt\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("sltu\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("xor\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("srl\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("sra\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("or\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("and\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        format!("slli\t{}, {}, {}", abi(rd), abi(rs1), shamt)
    }

    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        format!("srli\t{}, {}, {}", abi(rd), abi(rs1), shamt)
    }

    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        format!("srai\t{}, {}, {}", abi(rd), abi(rs1), shamt)
    }

    fn fence(&mut self, _fm: u32, _rd: Reg, _rs1: Reg) -> Self::Item {
        format!("fence")
    }

    fn ecall(&mut self) -> Self::Item {
        format!("ecall")
    }

    fn ebreak(&mut self) -> Self::Item {
        format!("ebreak")
    }
}

impl DecodeRv32m for Disassembler {
    type Item = String;

    fn mul(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("mul\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn mulh(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("mulh\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn mulhsu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("mulhsu\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn mulhu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("mulhu\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn div(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("div\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn divu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("divu\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn rem(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("rem\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }

    fn remu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        format!("remu\t{}, {}, {}", abi(rd), abi(rs1), abi(rs2))
    }
}
