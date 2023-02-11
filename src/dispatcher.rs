// TODO: this is for dispatching instructions and executing them, but it needs to be developed further.

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

// TODO: These exist for *testing*. Move them into a testing module - they don't even belong in `dispatcher.rs`.
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
