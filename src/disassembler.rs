use crate::extract::*;
use crate::Decoder;
use crate::{
    Bimm12Rs1Rs2::{self, *},
    Imm12RdRs1::{self, *},
    Imm12Rs1Rs2::{self, *},
    Imm20Rd, Jimm20Rd, NoArgs, RdFmPredRdRs1Succ,
    RdRs1::{self, *},
    RdRs1Rm,
    RdRs1Rs2::{self, *},
    RdRs1Rs2Rm, RdRs1Rs2Rs3Rm,
    RdRs1Shamtw::{self, *},
    Trap,
};

pub struct Disassembler {}

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

    fn trap(&mut self, opcode: Trap, ins: u32) -> Self::Item {
        // Illegal instruction.
        format!("{}\t0x{:04x}", opcode, ins)
    }

    fn no_args(&mut self, opcode: NoArgs, _ins: u32) -> Self::Item {
        // "ECALL"
        // "EBREAK"
        // "URET"
        // "SRET"
        // "MRET"
        format!("{}", opcode)
    }

    fn jimm20_rd(&mut self, opcode: Jimm20Rd, ins: u32) -> Self::Item {
        // "JAL %s, %d", abiNames[ins->rd_imm.rd], ins->rd_imm.imm
        format!(
            "{}\t{}, {}",
            opcode,
            extract_rd(ins),
            extract_jimmediate(ins)
        )
    }

    fn bimm12hi_bimm12lo_rs1_rs2(&mut self, opcode: Bimm12Rs1Rs2, ins: u32) -> Self::Item {
        match opcode {
            // "BEQ %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BNE %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BLT %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BGE %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BLTU %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            // "BGEU %s, %s, %d", abiNames[ins->rs1_rs2_imm.rs1], abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm
            Beq | Bne | Blt | Bge | Bltu | Bgeu => {
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

    fn rd_rm_rs1(&mut self, opcode: RdRs1Rm, ins: u32) -> Self::Item {
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

    fn rd_rm_rs1_rs2(&mut self, opcode: RdRs1Rs2Rm, ins: u32) -> Self::Item {
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

    fn rd_rs1(&mut self, opcode: RdRs1, ins: u32) -> Self::Item {
        // "FMV.X.W %s, %s", abiNames[ins->rd_rs1.rd], fabiNames[ins->rd_rs1.rs1])
        // "FMV.W.X %s, %s", fabiNames[ins->rd_rs1.rd], abiNames[ins->rd_rs1.rs1])
        // "FCLASS.S %s, %s", abiNames[ins->rd_rs1.rd], fabiNames[ins->rd_rs1.rs1])
        match opcode {
            FmvXW | FclassS => {
                format!(
                    "{}\t{}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::fabi_name(extract_rs1(ins))
                )
            }
            FmvWX => {
                format!(
                    "{}\t{}, {}",
                    opcode,
                    Disassembler::fabi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins))
                )
            }
        }
    }

    fn rd_rm_rs1_rs2_rs3(&mut self, opcode: RdRs1Rs2Rs3Rm, ins: u32) -> Self::Item {
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

    fn rd_rs1_rs2(&mut self, opcode: RdRs1Rs2, ins: u32) -> Self::Item {
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
            Add | Mul | Sub | Sll | Mulh | Slt | Mulhsu | Sltu | Mulhu | Xor | Div | Srl | Divu
            | Sra | Or | Rem | And | Remu => {
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
            FleS | FltS | FeqS => {
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
            FsgnjS | FminS | FsgnjnS | FmaxS | FsgnjxS => {
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

    fn imm12hi_imm12lo_rs1_rs2(&mut self, opcode: Imm12Rs1Rs2, ins: u32) -> Self::Item {
        match opcode {
            // "SB %s, %d(%s)", abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            // "SH %s, %d(%s)", abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            // "SW %s, %d(%s)", abiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            Sb | Sh | Sw => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::abi_name(extract_rs2(ins)),
                    extract_simmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }

            // "FSW %s, %d(%s)", fabiNames[ins->rs1_rs2_imm.rs2], ins->rs1_rs2_imm.imm, abiNames[ins->rs1_rs2_imm.rs1]
            Fsw => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::fabi_name(extract_rs2(ins)),
                    extract_simmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }
        }
    }

    fn imm20_rd(&mut self, opcode: Imm20Rd, ins: u32) -> Self::Item {
        // TRACE("AUIPC %s, %d\n", abiNames[ins->rd_imm.rd], ins->rd_imm.imm >> 12);
        // TRACE("LUI %s, %d\n", abiNames[ins->rd_imm.rd], ins->rd_imm.imm >> 12);
        format!(
            "{}\t{}, {}",
            opcode,
            Disassembler::abi_name(extract_rd(ins)),
            extract_uimmediate(ins) >> 12 // TODO: Does the shift belong here, or with extract_uimmediate()?
        )
    }

    fn rd_rs1_shamtw(&mut self, opcode: RdRs1Shamtw, ins: u32) -> Self::Item {
        match opcode {
            // "SLLI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SRLI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SRAI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            Slli | Srli | Srai => {
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

    fn fm_pred_rd_rs1_succ(&mut self, opcode: RdFmPredRdRs1Succ, _ins: u32) -> Self::Item {
        // "FENCE"
        // We're totally ignoring FENCE.TSO as it's optional.
        format!("{}\t", opcode)
    }

    fn imm12_rd_rs1(&mut self, opcode: Imm12RdRs1, ins: u32) -> Self::Item {
        match opcode {
            // "LB %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LH %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LW %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LBU %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            // "LHU %s, %d(%s)", abiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            Lb | Lh | Lw | Lbu | Lhu => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    extract_iimmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }
            // "FLW %s, %d(%s)", fabiNames[ins->rd_rs1_imm.rd], ins->rd_rs1_imm.imm, abiNames[ins->rd_rs1_imm.rs1]
            Flw => {
                format!(
                    "{}\t{}, {}({})",
                    opcode,
                    Disassembler::fabi_name(extract_rd(ins)),
                    extract_iimmediate(ins),
                    Disassembler::abi_name(extract_rs1(ins)),
                )
            }
            // "FENCE.I""
            FenceI => {
                format!("{}", opcode)
            }
            // "ADDI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SLTI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "SLTIU %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "XORI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "ORI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            // "ANDI %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            Addi | Slti | Sltiu | Xori | Ori | Andi => {
                format!(
                    "{}\t{}, {}, {}",
                    opcode,
                    Disassembler::abi_name(extract_rd(ins)),
                    Disassembler::abi_name(extract_rs1(ins)),
                    extract_iimmediate(ins)
                )
            }
            // "JALR %s, %s, %d", abiNames[ins->rd_rs1_imm.rd], abiNames[ins->rd_rs1_imm.rs1], ins->rd_rs1_imm.imm
            Jalr => {
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
