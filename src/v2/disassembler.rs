use super::{tobits::Reg, DecodeRv32c, DecodeRv32i, DecodeRv32m};

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

impl DecodeRv32c for Disassembler {
    type Item = String;
    
    fn c_addi4spn(&mut self, rdp: Reg, imm: u32) -> Self::Item {
        // addi rdp, x2, nzuimm[9:2]
        self.addi(rdp, Reg::Sp, imm)
    }

    fn c_lw(&mut self, rdp: Reg, rs1p: Reg, imm: u32) -> Self::Item {
        // lw rdp, offset[6:2](rs1p)
        self.lw(rdp, rs1p, imm)
    }

    fn c_sw(&mut self, rs1p: Reg, rs2p: Reg, imm: u32) -> Self::Item {
        //  sw rs2p, offset[6:2](rs1p)
        self.sw(rs1p, rs2p, imm)
    }

    fn c_sub(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // sub rdp, rdp, rs2p
        self.sub(rdrs1p, rdrs1p, rs2p)
    }

    fn c_xor(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // xor rdp, rdp, rs2p
        self.xor(rdrs1p, rdrs1p, rs2p)
    }

    fn c_or(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // or rdp, rdp, rs2p
        self.or(rdrs1p, rdrs1p, rs2p)
    }

    fn c_and(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // and rdp, rdp, rs2p
        self.and(rdrs1p, rdrs1p, rs2p)
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        format!("c.nop")
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        // addi x2, x2, nzimm[9:4]
        self.addi(Reg::Sp, Reg::Sp, imm)
    }

    fn c_andi(&mut self, rsrs1p: Reg, imm: u32) -> Self::Item {
        // andi rdp, rdp, imm[5:0]
        self.andi(rsrs1p, rsrs1p, imm)
    }

    fn c_addi(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // addi rd, rd, nzimm[5:0]
        self.addi(rdrs1n0, rdrs1n0, imm)
    }

    fn c_li(&mut self, rd: Reg, imm: u32) -> Self::Item {
        // addi rd, x0, imm[5:0]
        self.addi(rd, Reg::Zero, imm)
    }

    fn c_lui(&mut self, rdn2: Reg, imm: u32) -> Self::Item {
        // lui rd, nzimm[17:12]
        self.lui(rdn2, imm)
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        // jal x0, offset[11:1]
        self.jal(Reg::Zero, imm)
    }

    fn c_beqz(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // beq rs1p, x0, offset[8:1]
        self.beq(rs1p, Reg::Zero, imm)
    }

    fn c_bnez(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // bne rs1p, x0, offset[8:1]
        self.bne(rs1p, Reg::Zero, imm)
    }

    fn c_jr(&mut self, rs1n0: Reg) -> Self::Item {
        // jalr x0, 0(rs1)
        self.jalr(Reg::Zero, rs1n0, 0)
    }

    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item {
        // jalr x1, 0(rs1)
        self.jalr(Reg::Ra, rs1n0, 0)
    }

    fn c_ebreak(&mut self) -> Self::Item {
        self.ebreak()
    }

    fn c_mv(&mut self, rd: Reg, rs2n0: Reg) -> Self::Item {
        // add rd, x0, rs2
        self.add(rd, Reg::Zero, rs2n0)
    }

    fn c_add(&mut self, rdrs1: Reg, rs2n0: Reg) -> Self::Item {
        // add rd, rd, rs2
        self.add(rdrs1, rdrs1, rs2n0)
    }

    fn c_lwsp(&mut self, rdn0: Reg, imm: u32) -> Self::Item {
        // lw rd, offset[7:2](x2)
        self.lw(rdn0, Reg::Sp, imm)
    }

    fn c_swsp(&mut self, rs2: Reg, imm: u32) -> Self::Item {
        // sw rs2, offset[7:2](x2)
        self.sw(Reg::Sp, rs2, imm)
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        // jal x1, offset[11:1]
        self.jal(Reg::Ra, imm)
    }

    fn c_srli(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // srli rdp, drp, shamt[5:0]
        self.srli(rdrs1p, rdrs1p, imm)
    }

    fn c_srai(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // srai rdp, rdp, shamt[5:0]
        self.srai(rdrs1p, rdrs1p, imm)
    }

    fn c_slli(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // slli rd, rd, shamt[5:0]
        self.slli(rdrs1n0, rdrs1n0, imm)
    }
}
