use crate::extract::*;
use crate::Decoder;
use crate::{
    ExecFnBimm12Rs1Rs2, ExecFnImm12RdRs1, ExecFnImm12Rs1Rs2, ExecFnImm20Rd, ExecFnJimm20Rd,
    ExecFnNoArgs, ExecFnRdFmPredRdRs1Succ, ExecFnRdRs1, ExecFnRdRs1Rm, ExecFnRdRs1Rs2,
    ExecFnRdRs1Rs2Rm, ExecFnRdRs1Rs2Rs3Rm, ExecFnRdRs1Shamtw, ExecFnTrap,
};

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
