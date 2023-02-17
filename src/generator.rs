
use crate::{
    Bimm12Rs1Rs2, Imm12RdRs1, Imm12Rs1Rs2, Imm20Rd, Jimm20Rd, NoArgs, RdFmPredRdRs1Succ, RdRs1,
    RdRs1Rm, RdRs1Rs2, RdRs1Rs2Rm, RdRs1Rs2Rs3Rm, RdRs1Shamtw, Trap,
};
use crate::{Decoder};

#[derive(Debug)]
enum DecodedInstruction {
    NoArgs {
        instruction: NoArgs, // Which instructions are viable for these parameters.
    },
    Fdr {
        cache_line: u32, // The instruction's cache line.
        index: u32,      // The instruction's index in the cache line.
    },
    RdFmPredRdRs1Succ {
        instruction: RdFmPredRdRs1Succ, // Which instructions are viable for these parameters.
        fm: u8,                         // Fence "mode".
        rd: u8,                         // Destination register. Currently ignored.
        rs1: u8,                        // Source register. Currently ignored.
    },
    Imm20Rd {
        instruction: Imm20Rd, // Which instructions are viable for these parameters.
        rd: u8,               // Destination register.
        imm: i32,             // Immediate operand.
    },
    Jimm20Rd {
        instruction: Jimm20Rd, // Which instructions are viable for these parameters.
        rd: u8,                // Destination register.
        imm: i32,              // Immediate operand.
    },
    RdRs1 {
        instruction: RdRs1, // Which instructions are viable for these parameters.
        rd: u8,             // Destination register.
        rs1: u8,            // Source register.
    },
    Imm12RdRs1 {
        instruction: Imm12RdRs1, // Which instructions are viable for these parameters.
        rd: u8,                  // Destination register.
        rs1: u8,                 // Source register.
        imm: i32,                // Immediate operand.
    },
    RdRs1Shamtw {
        instruction: RdRs1Shamtw, // Which instructions are viable for these parameters.
        rd: u8,                   // Destination register.
        rs1: u8,                  // Source register.
        shamt: u8,                // Shift amount.
    },
    RdRs1Rs2 {
        instruction: RdRs1Rs2, // Which instructions are viable for these parameters.
        rd: u8,                // Destination register.
        rs1: u8,               // First source register.
        rs2: u8,               // Second source register.
    },
    Imm12Rs1Rs2 {
        instruction: Imm12Rs1Rs2, // Which instructions are viable for these parameters.
        rs1: u8,                  // First source register.
        rs2: u8,                  // Second source register.
        imm: i32,                 // Immediate operand.
    },
    Bimm12Rs1Rs2 {
        instruction: Bimm12Rs1Rs2, // Which instructions are viable for these parameters.
        rs1: u8,                   // First source register.
        rs2: u8,                   // Second source register.
        imm: i32,                  // Immediate operand.
    },
    RdRs1Rs2Rs3Rm {
        instruction: RdRs1Rs2Rs3Rm, // Which instructions are viable for these parameters.
        rd: u8,                     // Destination register.
        rs1: u8,                    // First source register.
        rs2: u8,                    // Second source register.
        rs3: u8,                    // Third source register.
        rm: u8,
    },
    RdRs1Rm {
        instruction: RdRs1Rm, // Which instructions are viable for these parameters.
        rd: u8,               // Destination register.
        rs1: u8,              // Source register.
        rm: u8,               // Rounding mode.
    },
    RdRs1Rs2Rm {
        instruction: RdRs1Rs2Rm, // Which instructions are viable for these parameters.
        rd: u8,                  // Destination register.
        rs1: u8,                 // First source register.
        rs2: u8,                 // Second source register.
        rm: u8,                  // Rounding mode.
    },
    Trap {
        instruction: Trap, // Which instructions are viable for these parameters.
        machine_code: u32,
    },
}

struct Generator;

// impl OldDecoder for Generator {
//     type Item = DecodedInstruction;

//     fn trap(&mut self, instruction: Trap, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::Trap {
//             instruction,
//             machine_code,
//         }
//     }

//     fn no_args(&mut self, instruction: NoArgs, _machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::NoArgs { instruction }
//     }

//     fn jimm20_rd(&mut self, instruction: Jimm20Rd, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::Jimm20Rd {
//             instruction: instruction,
//             rd: extract_rd(machine_code),
//             imm: extract_jimmediate(machine_code),
//         }
//     }

//     fn bimm12hi_bimm12lo_rs1_rs2(
//         &mut self,
//         instruction: Bimm12Rs1Rs2,
//         machine_code: u32,
//     ) -> DecodedInstruction {
//         DecodedInstruction::Bimm12Rs1Rs2 {
//             instruction,
//             rs1: extract_rs1(machine_code),
//             rs2: extract_rs2(machine_code),
//             imm: extract_bimmediate(machine_code),
//         }
//     }

//     fn rd_rm_rs1(&mut self, instruction: RdRs1Rm, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::RdRs1Rm {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//             rm: extract_rm(machine_code),
//         }
//     }

//     fn rd_rm_rs1_rs2(&mut self, instruction: RdRs1Rs2Rm, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::RdRs1Rs2Rm {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//             rs2: extract_rs2(machine_code),
//             rm: extract_rm(machine_code),
//         }
//     }

//     fn rd_rs1(&mut self, instruction: RdRs1, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::RdRs1 {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//         }
//     }

//     fn rd_rm_rs1_rs2_rs3(
//         &mut self,
//         instruction: RdRs1Rs2Rs3Rm,
//         machine_code: u32,
//     ) -> DecodedInstruction {
//         DecodedInstruction::RdRs1Rs2Rs3Rm {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//             rs2: extract_rs2(machine_code),
//             rs3: extract_rs3(machine_code),
//             rm: extract_rm(machine_code),
//         }
//     }

//     fn rd_rs1_rs2(&mut self, instruction: RdRs1Rs2, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::RdRs1Rs2 {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//             rs2: extract_rs2(machine_code),
//         }
//     }

//     fn imm12hi_imm12lo_rs1_rs2(
//         &mut self,
//         instruction: Imm12Rs1Rs2,
//         machine_code: u32,
//     ) -> DecodedInstruction {
//         DecodedInstruction::Imm12Rs1Rs2 {
//             instruction,
//             rs1: extract_rs1(machine_code),
//             rs2: extract_rs2(machine_code),
//             imm: extract_simmediate(machine_code),
//         }
//     }

//     fn imm20_rd(&mut self, instruction: Imm20Rd, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::Imm20Rd {
//             instruction,
//             rd: extract_rd(machine_code),
//             imm: extract_uimmediate(machine_code),
//         }
//     }

//     fn rd_rs1_shamtw(&mut self, instruction: RdRs1Shamtw, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::RdRs1Shamtw {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//             imm: extract_iimmediate(machine_code),
//         }
//     }

//     fn fm_pred_rd_rs1_succ(
//         &mut self,
//         instruction: RdFmPredRdRs1Succ,
//         machine_code: u32,
//     ) -> DecodedInstruction {
//         DecodedInstruction::RdFmPredRdRs1Succ {
//             instruction,
//             fm: extract_fm(machine_code),
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//         }
//     }

//     fn imm12_rd_rs1(&mut self, instruction: Imm12RdRs1, machine_code: u32) -> DecodedInstruction {
//         DecodedInstruction::Imm12RdRs1 {
//             instruction,
//             rd: extract_rd(machine_code),
//             rs1: extract_rs1(machine_code),
//             imm: extract_iimmediate(machine_code),
//         }
//     }
// }

impl Decoder for Generator {
    type Item = DecodedInstruction;

    fn trap(&mut self, instruction: Trap, machine_code: u32) -> Self::Item {
        DecodedInstruction::Trap {
            instruction,
            machine_code,
        }
    }

    fn b_type(&mut self, instruction: Bimm12Rs1Rs2, bimm: i32, rs1: u8, rs2: u8) -> Self::Item {
        DecodedInstruction::Bimm12Rs1Rs2 {
            instruction,
            rs1,
            rs2,
            imm: bimm,
        }
    }

    fn fence(&mut self, instruction: RdFmPredRdRs1Succ, fm: u8, rd: u8, rs1: u8) -> Self::Item {
        DecodedInstruction::RdFmPredRdRs1Succ {
            instruction,
            fm,
            rd,
            rs1,
        }
    }

    fn i_type(&mut self, instruction: Imm12RdRs1, iimm: i32, rd: u8, rs1: u8) -> Self::Item {
        DecodedInstruction::Imm12RdRs1 {
            instruction,
            rd,
            rs1,
            imm: iimm,
        }
    }

    fn s_type(&mut self, instruction: Imm12Rs1Rs2, simm: i32, rs1: u8, rs2: u8) -> Self::Item {
        DecodedInstruction::Imm12Rs1Rs2 {
            instruction,
            rs1,
            rs2,
            imm: simm,
        }
    }

    fn u_type(&mut self, instruction: Imm20Rd, uimm: i32, rd: u8) -> Self::Item {
        DecodedInstruction::Imm20Rd {
            instruction,
            rd,
            imm: uimm,
        }
    }

    fn j_type(&mut self, instruction: Jimm20Rd, jimm: i32, rd: u8) -> Self::Item {
        DecodedInstruction::Jimm20Rd {
            instruction,
            rd,
            imm: jimm,
        }
    }

    fn no_args(&mut self, instruction: NoArgs) -> Self::Item {
        DecodedInstruction::NoArgs { instruction }
    }

    fn rd_rm_rs1(&mut self, instruction: RdRs1Rm, rd: u8, rm: u8, rs1: u8) -> Self::Item {
        DecodedInstruction::RdRs1Rm {
            instruction,
            rd,
            rs1,
            rm,
        }
    }

    fn rd_rm_rs1_rs2(
        &mut self,
        instruction: RdRs1Rs2Rm,
        rd: u8,
        rm: u8,
        rs1: u8,
        rs2: u8,
    ) -> Self::Item {
        DecodedInstruction::RdRs1Rs2Rm {
            instruction,
            rd,
            rs1,
            rs2,
            rm,
        }
    }

    fn rd_rm_rs1_rs2_rs3(
        &mut self,
        instruction: RdRs1Rs2Rs3Rm,
        rd: u8,
        rm: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
    ) -> Self::Item {
        DecodedInstruction::RdRs1Rs2Rs3Rm {
            instruction,
            rd,
            rs1,
            rs2,
            rs3,
            rm,
        }
    }

    fn rd_rs1(&mut self, instruction: RdRs1, rd: u8, rs1: u8) -> Self::Item {
        DecodedInstruction::RdRs1 {
            instruction,
            rd,
            rs1,
        }
    }

    fn rd_rs1_rs2(&mut self, instruction: RdRs1Rs2, rd: u8, rs1: u8, rs2: u8) -> Self::Item {
        DecodedInstruction::RdRs1Rs2 {
            instruction,
            rd,
            rs1,
            rs2,
        }
    }

    fn rd_rs1_shamtw(
        &mut self,
        instruction: RdRs1Shamtw,
        rd: u8,
        rs1: u8,
        shamt: u8,
    ) -> Self::Item {
        DecodedInstruction::RdRs1Shamtw {
            instruction,
            rd,
            rs1,
            shamt,
        }
    }
}
