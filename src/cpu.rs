use crate::{
    BasicMem, BasicTrapHandler, Bimm12Rs1Rs2, Decoder, Imm12RdRs1, Imm12Rs1Rs2, Imm20Rd, Jimm20Rd,
    Mem, NoArgs, RdFmPredRdRs1Succ, RdRs1, RdRs1Rm, RdRs1Rs2, RdRs1Rs2Rm, RdRs1Rs2Rs3Rm,
    RdRs1Shamtw, Trap, TrapCause, TrapHandler,
};

// We have a CPU that owns some memory and a trap handler.
// TODO: should the fields be public? I've done so for convenience.
pub struct Cpu<T, U>
where
    T: Mem,
    U: TrapHandler,
{
    pub pc: u32,           // The program counter.
    pub xreg: [u32; 32],   // Regular registerx, x0-x31.
    pub freg: [f32; 32],   // Floating point registers, f0-f31.
    pub mcause: TrapCause, // The machine cause register, written to when a trap is taken into M-mode.
    pub mtval: u32,        // The machine trap value register. Contains trap-specific informatin.
    pub mem: T,            // Memory.
    pub trap_handler: U,   // Trap handler.
}

// A Cpu implementation that uses BasicMem and BasicTrapHandler.
impl Cpu<BasicMem, BasicTrapHandler> {
    pub fn new() -> Self {
        Self::with_mem(BasicMem::new())
    }

    pub fn with_mem(mem: BasicMem) -> Self {
        Self {
            pc: 0,
            xreg: Default::default(),
            freg: Default::default(),
            mcause: TrapCause::Breakpoint,
            mtval: 0,
            mem: mem,
            trap_handler: BasicTrapHandler::new(),
        }
    }
}

pub type BasicCpu = Cpu<BasicMem, BasicTrapHandler>;

// A Decoder implementation for any Cpu.
impl<T, U> Decoder for Cpu<T, U>
where
    T: Mem,
    U: TrapHandler<Item = ()>,
{
    type Item = ();

    fn trap(&mut self, instruction: Trap, machine_code: u32) -> Self::Item {
        self.trap_handler.handle_trap(TrapCause::IllegalInstruction);
    }

    fn b_type(&mut self, instruction: Bimm12Rs1Rs2, bimm: i32, rs1: u8, rs2: u8) -> Self::Item {
        let bimm = bimm as u32;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            Bimm12Rs1Rs2::Beq => self.beq(rs1, rs2, bimm),
            Bimm12Rs1Rs2::Bne => self.bne(rs1, rs2, bimm),
            Bimm12Rs1Rs2::Blt => self.blt(rs1, rs2, bimm),
            Bimm12Rs1Rs2::Bge => self.bge(rs1, rs2, bimm),
            Bimm12Rs1Rs2::Bltu => self.bltu(rs1, rs2, bimm),
            Bimm12Rs1Rs2::Bgeu => self.bgeu(rs1, rs2, bimm),
        }
    }

    fn fence(&mut self, _instruction: RdFmPredRdRs1Succ, _fm: u8, _rd: u8, _rs1: u8) -> Self::Item {
        // nop
    }

    fn i_type(&mut self, instruction: Imm12RdRs1, iimm: i32, rd: u8, rs1: u8) -> Self::Item {
        let iimm = iimm as u32;
        let rd = rd as usize;
        let rs1 = rs1 as usize;
        match instruction {
            Imm12RdRs1::Lb => self.lb(rd, rs1, iimm),
            Imm12RdRs1::Lh => self.lh(rd, rs1, iimm),
            Imm12RdRs1::Lw => self.lw(rd, rs1, iimm),
            Imm12RdRs1::Lbu => self.lbu(rd, rs1, iimm),
            Imm12RdRs1::Lhu => self.lhu(rd, rs1, iimm),
            Imm12RdRs1::Flw => self.flw(rd, rs1, iimm),
            Imm12RdRs1::Addi => self.addi(rd, rs1, iimm),
            Imm12RdRs1::Slti => self.slti(rd, rs1, iimm),
            Imm12RdRs1::Sltiu => self.sltiu(rd, rs1, iimm),
            Imm12RdRs1::Xori => self.xori(rd, rs1, iimm),
            Imm12RdRs1::Ori => self.ori(rd, rs1, iimm),
            Imm12RdRs1::Andi => self.andi(rd, rs1, iimm),
            Imm12RdRs1::Jalr => self.jalr(rd, rs1, iimm),
        }
    }

    fn s_type(&mut self, instruction: Imm12Rs1Rs2, simm: i32, rs1: u8, rs2: u8) -> Self::Item {
        let simm = simm as u32;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            Imm12Rs1Rs2::Sb => self.sb(rs1, rs2, simm),
            Imm12Rs1Rs2::Sh => self.sh(rs1, rs2, simm),
            Imm12Rs1Rs2::Sw => self.sw(rs1, rs2, simm),
            Imm12Rs1Rs2::Fsw => self.fsw(rs1, rs2, simm),
        }
    }

    fn u_type(&mut self, instruction: Imm20Rd, uimm: i32, rd: u8) -> Self::Item {
        let uimm = uimm as u32;
        let rd = rd as usize;
        match instruction {
            Imm20Rd::Auipc => self.auipc(rd, uimm),
            Imm20Rd::Lui => self.lui(rd, uimm),
        }
    }

    fn j_type(&mut self, instruction: Jimm20Rd, jimm: i32, rd: u8) -> Self::Item {
        let jimm = jimm as u32;
        let rd = rd as usize;
        match instruction {
            Jimm20Rd::Jal => self.jal(rd, jimm),
        }
    }

    fn no_args(&mut self, instruction: NoArgs) -> Self::Item {
        match instruction {
            NoArgs::Ecall => self.trap_handler.handle_ecall(),
            NoArgs::Ebreak => self.trap_handler.handle_ebreak(),
        }
    }

    fn rd_rm_rs1(&mut self, instruction: RdRs1Rm, rd: u8, rm: u8, rs1: u8) -> Self::Item {
        let rd = rd as usize;
        let rm = rm as usize;
        let rs1 = rs1 as usize;
        match instruction {
            RdRs1Rm::FsqrtS => self.fsqrt_s(rd, rs1, rm),
            RdRs1Rm::FcvtWS => self.fcvt_w_s(rd, rs1, rm),
            RdRs1Rm::FcvtWuS => self.fcvt_wu_s(rd, rs1, rm),
            RdRs1Rm::FcvtSW => self.fcvt_s_w(rd, rs1, rm),
            RdRs1Rm::FcvtSWu => self.fcvt_s_wu(rd, rs1, rm),
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
        let rd = rd as usize;
        let rm = rm as usize;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            RdRs1Rs2Rm::FaddS => self.fadd_s(rd, rs1, rs2, rm),
            RdRs1Rs2Rm::FsubS => self.fsub_s(rd, rs1, rs2, rm),
            RdRs1Rs2Rm::FmulS => self.fmul_s(rd, rs1, rs2, rm),
            RdRs1Rs2Rm::FdivS => self.fdiv_s(rd, rs1, rs2, rm),
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
        let rd = rd as usize;
        let rm = rm as usize;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        let rs3 = rs3 as usize;
        match instruction {
            RdRs1Rs2Rs3Rm::FmaddS => self.fmadd_s(rd, rs1, rs2, rs3, rm),
            RdRs1Rs2Rs3Rm::FmsubS => self.fmsub_s(rd, rs1, rs2, rs3, rm),
            RdRs1Rs2Rs3Rm::FnmsubS => self.fnmsub_s(rd, rs1, rs2, rs3, rm),
            RdRs1Rs2Rs3Rm::FnmaddS => self.fnmadd_s(rd, rs1, rs2, rs3, rm),
        }
    }

    fn rd_rs1(&mut self, instruction: RdRs1, rd: u8, rs1: u8) -> Self::Item {
        let rd = rd as usize;
        let rs1 = rs1 as usize;
        match instruction {
            RdRs1::FmvXW => self.fmv_x_w(rd, rs1),
            RdRs1::FmvWX => self.fmv_w_x(rd, rs1),
            RdRs1::FclassS => self.fclass_s(rd, rs1),
        }
    }

    fn rd_rs1_rs2(&mut self, instruction: RdRs1Rs2, rd: u8, rs1: u8, rs2: u8) -> Self::Item {
        let rd = rd as usize;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            RdRs1Rs2::Add => self.add(rd, rs1, rs2),
            RdRs1Rs2::Mul => self.mul(rd, rs1, rs2),
            RdRs1Rs2::Sub => self.sub(rd, rs1, rs2),
            RdRs1Rs2::Sll => self.sll(rd, rs1, rs2),
            RdRs1Rs2::Mulh => self.mulh(rd, rs1, rs2),
            RdRs1Rs2::Slt => self.slt(rd, rs1, rs2),
            RdRs1Rs2::Mulhsu => self.mulhsu(rd, rs1, rs2),
            RdRs1Rs2::Sltu => self.sltu(rd, rs1, rs2),
            RdRs1Rs2::Mulhu => self.mulhu(rd, rs1, rs2),
            RdRs1Rs2::Xor => self.xor(rd, rs1, rs2),
            RdRs1Rs2::Div => self.div(rd, rs1, rs2),
            RdRs1Rs2::Srl => self.srl(rd, rs1, rs2),
            RdRs1Rs2::Divu => self.divu(rd, rs1, rs2),
            RdRs1Rs2::Sra => self.sra(rd, rs1, rs2),
            RdRs1Rs2::Or => self.or(rd, rs1, rs2),
            RdRs1Rs2::Rem => self.rem(rd, rs1, rs2),
            RdRs1Rs2::And => self.and(rd, rs1, rs2),
            RdRs1Rs2::Remu => self.remu(rd, rs1, rs2),
            RdRs1Rs2::FsgnjS => self.fsgnj_s(rd, rs1, rs2),
            RdRs1Rs2::FminS => self.fmin_s(rd, rs1, rs2),
            RdRs1Rs2::FleS => self.fle_s(rd, rs1, rs2),
            RdRs1Rs2::FsgnjnS => self.fsgnjn_s(rd, rs1, rs2),
            RdRs1Rs2::FmaxS => self.fmax_s(rd, rs1, rs2),
            RdRs1Rs2::FltS => self.flt_s(rd, rs1, rs2),
            RdRs1Rs2::FsgnjxS => self.fsgnjx_s(rd, rs1, rs2),
            RdRs1Rs2::FeqS => self.feq_s(rd, rs1, rs2),
        }
    }

    fn rd_rs1_shamtw(
        &mut self,
        instruction: RdRs1Shamtw,
        rd: u8,
        rs1: u8,
        shamt: u8,
    ) -> Self::Item {
        let rd = rd as usize;
        let rs1 = rs1 as usize;
        let shamt = shamt as u32;
        match instruction {
            RdRs1Shamtw::Slli => self.slli(rd, rs1, shamt),
            RdRs1Shamtw::Srli => self.srli(rd, rs1, shamt),
            RdRs1Shamtw::Srai => self.srai(rd, rs1, shamt),
        }
    }
}

trait Rv32i {
    // B-type instructions.
    fn beq(&mut self, rs1: usize, rs2: usize, bimm: u32);
    fn bne(&mut self, rs1: usize, rs2: usize, bimm: u32);
    fn blt(&mut self, rs1: usize, rs2: usize, bimm: u32);
    fn bge(&mut self, rs1: usize, rs2: usize, bimm: u32);
    fn bltu(&mut self, rs1: usize, rs2: usize, bimm: u32);
    fn bgeu(&mut self, rs1: usize, rs2: usize, bimm: u32);

    // I-type instructions.
    fn lb(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn lh(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn lw(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn lbu(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn lhu(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn addi(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn slti(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn sltiu(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn xori(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn ori(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn andi(&mut self, rd: usize, rs1: usize, iimm: u32);
    fn jalr(&mut self, rd: usize, rs1: usize, iimm: u32);

    // S-type instructions.
    fn sb(&mut self, rs1: usize, rs2: usize, simm: u32);
    fn sh(&mut self, rs1: usize, rs2: usize, simm: u32);
    fn sw(&mut self, rs1: usize, rs2: usize, simm: u32);

    // U-type instructions.
    fn auipc(&mut self, rd: usize, uimm: u32);
    fn lui(&mut self, rd: usize, uimm: u32);

    // J-type instructions.
    fn jal(&mut self, rd: usize, jimm: u32);

    // Arithmetic instructions.
    fn add(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn mul(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn sll(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn mulh(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn slt(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn mulhsu(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn mulhu(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn div(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn srl(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn divu(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn sra(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn or(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn rem(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn and(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn remu(&mut self, rd: usize, rs1: usize, rs2: usize);

    // Immediate shift instructions.
    fn slli(&mut self, rd: usize, rs1: usize, shamt: u32);
    fn srli(&mut self, rd: usize, rs1: usize, shamt: u32);
    fn srai(&mut self, rd: usize, rs1: usize, shamt: u32);
}

impl<T, U> Rv32i for Cpu<T, U>
where
    T: Mem,
    U: TrapHandler<Item = ()>,
{
    // B-type instructions.

    fn beq(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // pc <- pc + ((rs1 == rs2) ? imm_b : 4)
        let offset = if self.xreg[rs1] == self.xreg[rs2] {
            bimm
        } else {
            4
        };
        self.pc = self.pc.wrapping_add(offset);
    }

    fn bne(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // pc <- pc + ((rs1 != rs2) ? imm_b : 4)
        let offset = if self.xreg[rs1] != self.xreg[rs2] {
            bimm
        } else {
            4
        };
        self.pc = self.pc.wrapping_add(offset);
    }

    fn blt(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Signed.
        // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
        let offset = if (self.xreg[rs1] as i32) < (self.xreg[rs2] as i32) {
            bimm
        } else {
            4
        };
        self.pc = self.pc.wrapping_add(offset);
    }

    fn bge(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Signed.
        // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
        let offset = if (self.xreg[rs1] as i32) >= (self.xreg[rs2] as i32) {
            bimm
        } else {
            4
        };
        self.pc = self.pc.wrapping_add(offset);
    }

    fn bltu(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Unsigned.
        // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
        let offset = if self.xreg[rs1] < self.xreg[rs2] {
            bimm
        } else {
            4
        };
        self.pc = self.pc.wrapping_add(offset);
    }

    fn bgeu(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Unsigned.
        // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
        let offset = if self.xreg[rs1] >= self.xreg[rs2] {
            bimm
        } else {
            4
        };
        self.pc = self.pc.wrapping_add(offset);
    }

    // I-type instructions.

    fn lb(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m8(rs1 + imm_i)), pc += 4
        match self.mem.read8(self.xreg[rs1].wrapping_add(iimm)) {
            Ok(byte) => {
                self.xreg[rd] = (((byte as i8) as i16) as i32) as u32; // TODO: this should be a function.
                self.pc = self.pc.wrapping_add(4);
                self.xreg[0] = 0;
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lh(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m16(rs1 + imm_i)), pc += 4
        match self.mem.read16(self.xreg[rs1].wrapping_add(iimm)) {
            Ok(half_word) => {
                self.xreg[rd] = ((half_word as i16) as i32) as u32; // TODO: this should be a function.
                self.pc = self.pc.wrapping_add(4);
                self.xreg[0] = 0;
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lw(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m32(rs1 + imm_i)), pc += 4
        match self.mem.read32(self.xreg[rs1].wrapping_add(iimm)) {
            Ok(word) => {
                self.xreg[rd] = word;
                self.pc = self.pc.wrapping_add(4);
                self.xreg[0] = 0;
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lbu(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- zx(m8(rs1 + imm_i)), pc += 4
        match self.mem.read8(self.xreg[rs1].wrapping_add(iimm)) {
            Ok(byte) => {
                self.xreg[rd] = byte as u32;
                self.pc = self.pc.wrapping_add(4);
                self.xreg[0] = 0;
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lhu(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- zx(m16(rs1 + imm_i)), pc += 4
        match self.mem.read16(self.xreg[rs1].wrapping_add(iimm)) {
            Ok(half_word) => {
                self.xreg[rd] = half_word as u32;
                self.pc = self.pc.wrapping_add(4);
                self.xreg[0] = 0;
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn addi(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 + imm_i, pc += 4
        self.xreg[rd] = self.xreg[rs1].wrapping_add(iimm);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn slti(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // Signed.
        // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
        let xreg_rs1 = self.xreg[rs1] as i32;
        let iimm = iimm as i32;
        self.xreg[rd] = if xreg_rs1 < iimm { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn sltiu(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // Unsigned.
        // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
        self.xreg[rd] = if self.xreg[rs1] < iimm { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn xori(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 ^ imm_i, pc += 4
        self.xreg[rd] = self.xreg[rs1] ^ iimm;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn ori(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 | imm_i, pc += 4
        self.xreg[rd] = self.xreg[rs1] | iimm;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn andi(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 & imm_i, pc += 4
        self.xreg[rd] = self.xreg[rs1] & iimm;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn jalr(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- pc + 4, pc <- (rs1 + imm_i) & ~1
        let rs1_before = self.xreg[rs1]; // Because rd and rs1 might be the same register.
        self.xreg[rd] = self.pc.wrapping_add(4);
        self.pc = rs1_before.wrapping_add(iimm) & !1;
        self.xreg[0] = 0;
    }

    // S-type instructions.

    fn sb(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // m8(rs1 + imm_s) <- rs2[7:0], pc += 4
        match self.mem.write8(
            self.xreg[rs1].wrapping_add(simm),
            (self.xreg[rs2] & 0xff) as u8,
        ) {
            Ok(_) => {
                self.pc = self.pc.wrapping_add(4);
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    fn sh(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // m16(rs1 + imm_s) <- rs2[15:0], pc += 4
        match self.mem.write16(
            self.xreg[rs1].wrapping_add(simm),
            (self.xreg[rs2] & 0xffff) as u16,
        ) {
            Ok(_) => {
                self.pc = self.pc.wrapping_add(4);
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    fn sw(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // m32(rs1 + imm_s) <- rs2[31:0], pc += 4
        match self
            .mem
            .write32(self.xreg[rs1].wrapping_add(simm), self.xreg[rs2])
        {
            Ok(_) => {
                self.pc = self.pc.wrapping_add(4);
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    // U-type instructions.

    fn auipc(&mut self, rd: usize, uimm: u32) {
        // rd <- pc + imm_u, pc += 4
        self.xreg[rd] = self.pc.wrapping_add(uimm);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn lui(&mut self, rd: usize, uimm: u32) {
        // rd <- imm_u, pc += 4
        self.xreg[rd] = uimm;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    // J-type instructions.

    fn jal(&mut self, rd: usize, jimm: u32) {
        // rd <- pc + 4, pc <- pc + imm_j
        self.xreg[rd] = self.pc.wrapping_add(4);
        self.pc = self.pc.wrapping_add(jimm);
        self.xreg[0] = 0;
    }

    // Arithmetic instructions.

    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 + rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1].wrapping_add(self.xreg[rs2]);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn mul(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 * rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1].wrapping_mul(self.xreg[rs2]);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 - rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1].wrapping_sub(self.xreg[rs2]);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 << (rs2 % XLEN), pc += 4
        self.xreg[rd] = self.xreg[rs1] << (self.xreg[rs2] % 32);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn mulh(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let xreg_rs1 = (self.xreg[rs1] as i32) as i64;
        let xreg_rs2 = (self.xreg[rs2] as i32) as i64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.xreg[rd] = t as u32;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // Signed.
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.xreg[rs1] as i32;
        let xreg_rs2 = self.xreg[rs2] as i32;
        self.xreg[rd] = if xreg_rs1 < xreg_rs2 { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn mulhsu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let xreg_rs1 = (self.xreg[rs1] as i32) as i64;
        let xreg_rs2 = (self.xreg[rs2] as u64) as i64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.xreg[rd] = t as u32;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.xreg[rs1];
        let xreg_rs2 = self.xreg[rs2];
        self.xreg[rd] = if xreg_rs1 < xreg_rs2 { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn mulhu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let xreg_rs1 = self.xreg[rs1] as u64;
        let xreg_rs2 = self.xreg[rs2] as u64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.xreg[rd] = t as u32;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 ^ rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1] ^ self.xreg[rs2];
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn div(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let dividend = self.xreg[rs1] as i32;
        let divisor = self.xreg[rs2] as i32;
        // Check for signed division overflow.
        if ((dividend as u32) != 0x80000000) || divisor != -1 {
            self.xreg[rd] = if divisor != 0 {
                (dividend.wrapping_div(divisor)) as u32
            } else {
                u32::MAX // -1.
            }
        } else {
            // Signed division overflow occurred.
            self.xreg[rd] = dividend as u32;
        }
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        self.xreg[rd] = self.xreg[rs1] >> (self.xreg[rs2] % 32);
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn divu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let dividend = self.xreg[rs1];
        let divisor = self.xreg[rs2];
        self.xreg[rd] = if divisor != 0 {
            dividend.wrapping_div(divisor)
        } else {
            u32::MAX // -1.
        };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        let xreg_rs1 = self.xreg[rs1] as i32;
        let shift = (self.xreg[rs2] % 32) as i32;
        self.xreg[rd] = (xreg_rs1 >> shift) as u32;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 | rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1] | self.xreg[rs2];
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn rem(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let dividend = self.xreg[rs1] as i32;
        let divisor = self.xreg[rs2] as i32;
        // Check for signed division overflow.
        if ((dividend as u32) != 0x80000000) || divisor != -1 {
            self.xreg[rd] = if divisor != 0 {
                (dividend % divisor) as u32
            } else {
                dividend as u32
            }
        } else {
            // Signed division overflow occurred.
            self.xreg[rd] = 0;
        }
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 & rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1] & self.xreg[rs2];
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn remu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let dividend = self.xreg[rs1];
        let divisor = self.xreg[rs2];
        self.xreg[rd] = if divisor != 0 {
            dividend % divisor
        } else {
            dividend
        };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    // Immediate shift instructions.

    fn slli(&mut self, rd: usize, rs1: usize, shamt: u32) {
        self.xreg[rd] = self.xreg[rs1] << shamt;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn srli(&mut self, rd: usize, rs1: usize, shamt: u32) {
        self.xreg[rd] = self.xreg[rs1] >> shamt;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn srai(&mut self, rd: usize, rs1: usize, shamt: u32) {
        let xreg_rs = self.xreg[rs1] as i32;
        let shamt = shamt as i32;
        self.xreg[rd] = (xreg_rs >> shamt) as u32;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }
}

trait Rv32f {
    // I-type instructions.
    fn flw(&mut self, rd: usize, rs1: usize, iimm: u32);

    // S-type instructions.
    fn fsw(&mut self, rs1: usize, rs2: usize, simm: u32);

    // Instructions with rd rs1 rm operands.
    fn fsqrt_s(&mut self, rd: usize, rs1: usize, rm: usize);
    fn fcvt_w_s(&mut self, rd: usize, rs1: usize, rm: usize);
    fn fcvt_wu_s(&mut self, rd: usize, rs1: usize, rm: usize);
    fn fcvt_s_w(&mut self, rd: usize, rs1: usize, rm: usize);
    fn fcvt_s_wu(&mut self, rd: usize, rs1: usize, rm: usize);

    // Arithmetic instructions.
    fn fadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize);
    fn fsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize);
    fn fmul_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize);
    fn fdiv_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize);

    // Fused multiply / add instructions.
    fn fmadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize);
    fn fmsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize);
    fn fnmsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize);
    fn fnmadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize);

    // Instructions with rd rs1 operands.
    fn fmv_x_w(&mut self, rd: usize, rs1: usize);
    fn fmv_w_x(&mut self, rd: usize, rs1: usize);
    fn fclass_s(&mut self, rd: usize, rs1: usize);

    // Instructions with rd rs1 rs2 operands.
    fn fsgnj_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn fmin_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn fle_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn fsgnjn_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn fmax_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn flt_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn fsgnjx_s(&mut self, rd: usize, rs1: usize, rs2: usize);
    fn feq_s(&mut self, rd: usize, rs1: usize, rs2: usize);
}

impl<T, U> Rv32f for Cpu<T, U>
where
    T: Mem,
    U: TrapHandler<Item = ()>,
{
    // I-type instructions.

    fn flw(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- f32(rs1 + imm_i)
        match self.mem.read32(self.xreg[rs1].wrapping_add(iimm)) {
            Ok(word) => {
                self.freg[rd] = f32::from_bits(word);
                self.pc = self.pc.wrapping_add(4);
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    // S-type instructions.

    fn fsw(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // f32(rs1 + imm_s) = rs2
        let data = f32::to_bits(self.freg[rs2]);
        match self.mem.write32(self.xreg[rs1].wrapping_add(simm), data) {
            Ok(_) => {
                self.pc = self.pc.wrapping_add(4);
            }
            Err(_) => {
                self.trap_handler.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    // Instructions with rd rs1 rm operands.

    fn fsqrt_s(&mut self, rd: usize, rs1: usize, _rm: usize) {
        // rd <- sqrt(rs1)
        let f = self.freg[rs1];
        self.freg[rd] = f32::sqrt(f);
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fcvt_w_s(&mut self, rd: usize, rs1: usize, _rm: usize) {
        // rd <- int32_t(rs1)
        let i = self.freg[rs1] as i32;
        self.xreg[rd] = i as u32;
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fcvt_wu_s(&mut self, rd: usize, rs1: usize, _rm: usize) {
        // rd <- uint32_t(rs1)
        let i = self.freg[rs1] as u32;
        self.xreg[rd] = i;
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fcvt_s_w(&mut self, rd: usize, rs1: usize, _rm: usize) {
        // rd <- float(int32_t((rs1))
        let i = self.xreg[rs1] as i32;
        self.freg[rd] = i as f32;
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fcvt_s_wu(&mut self, rd: usize, rs1: usize, _rm: usize) {
        // rd <- float(rs1)
        self.freg[rd] = self.xreg[rs1] as f32;
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    // Arithmetic instructions.

    fn fadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, _rm: usize) {
        // rd <- rs1 + rs2
        self.freg[rd] = self.freg[rs1] + self.freg[rs2];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, _rm: usize) {
        // rd <- rs1 - rs2
        self.freg[rd] = self.freg[rs1] - self.freg[rs2];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fmul_s(&mut self, rd: usize, rs1: usize, rs2: usize, _rm: usize) {
        // rd <- rs1 * rs2
        self.freg[rd] = self.freg[rs1] * self.freg[rs2];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fdiv_s(&mut self, rd: usize, rs1: usize, rs2: usize, _rm: usize) {
        // rd <- rs1 / rs2
        self.freg[rd] = self.freg[rs1] / self.freg[rs2];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    // Fused multiply / add instructions.

    fn fmadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, _rm: usize) {
        // rd <- (rs1 * rs2) + rs3
        self.freg[rd] = (self.freg[rs1] * self.freg[rs2]) + self.freg[rs3];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fmsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, _rm: usize) {
        // rd <- (rs1 * rs2) - rs3
        self.freg[rd] = (self.freg[rs1] * self.freg[rs2]) - self.freg[rs3];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fnmsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, _rm: usize) {
        // rd <- -(rs1 * rs2) + rs3
        self.freg[rd] = -(self.freg[rs1] * self.freg[rs2]) + self.freg[rs3];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    fn fnmadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, _rm: usize) {
        // rd <- -(rs1 * rs2) - rs3
        self.freg[rd] = -(self.freg[rs1] * self.freg[rs2]) - self.freg[rs3];
        self.pc = self.pc.wrapping_add(4);
        // TODO: handle rounding modes.
    }

    // Instructions with rd rs1 operands.

    fn fmv_x_w(&mut self, rd: usize, rs1: usize) {
        // bits(rd) <- bits(rs1)
        self.xreg[rd] = f32::to_bits(self.freg[rs1]);
        self.pc = self.pc.wrapping_add(4);

        self.xreg[0] = 0;
    }

    fn fmv_w_x(&mut self, rd: usize, rs1: usize) {
        // bits(rd) <- bits(rs1)
        self.freg[rd] = f32::from_bits(self.xreg[rs1]);
        self.pc = self.pc.wrapping_add(4);
    }

    fn fclass_s(&mut self, rd: usize, rs1: usize) {
        let v = self.freg[rs1];
        let bits = f32::to_bits(v);
        let result: u32 = if v == f32::NEG_INFINITY {
            1 << 0
        } else if v == f32::INFINITY {
            1 << 7
        } else if bits == 0x80000000 {
            // Negative zero.
            1 << 3
        } else if v == 0.0 {
            1 << 4
        } else if (bits & 0x7f800000) == 0 {
            // The exponent is zero.
            if (bits & 0x80000000) != 0 {
                // Negative subnormal number.
                1 << 2
            } else {
                // Postive subnormal number.
                1 << 5
            }
        } else if (bits & 0x7f800000) == 0x7f800000 && (bits & 0x00400000) != 0 {
            // Quiet NaN.
            1 << 9
        } else if (bits & 0x7f800000) == 0x7f800000 && (bits & 0x003fffff) != 0 {
            // Signalling NaN.
            1 << 8
        } else if v < 0.0 {
            1 << 1
        } else if v > 0.0 {
            1 << 6
        } else {
            0
        };
        self.xreg[rd] = result;
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    // Instructions with rd rs1 rs2 operands.

    fn fsgnj_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- abs(rs1) * sgn(rs2)
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.freg[rd] = freg_rs1.abs() * if freg_rs2 < 0.0 { -1.0 } else { 1.0 };
        self.pc = self.pc.wrapping_add(4);
    }

    fn fmin_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- min(rs1, rs2)
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.freg[rd] = freg_rs1.min(freg_rs2);
        self.pc = self.pc.wrapping_add(4);
    }

    fn fle_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- (rs1 <= rs2) ? 1 : 0;
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.xreg[rd] = if freg_rs1 < freg_rs2 { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn fsgnjn_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- abs(rs1) * -sgn(rs2)
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.freg[rd] = freg_rs1.abs() * if freg_rs2 < 0.0 { 1.0 } else { -1.0 };
        self.pc = self.pc.wrapping_add(4);
    }

    fn fmax_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- max(rs1, rs2)
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.freg[rd] = freg_rs1.max(freg_rs2);
        self.pc = self.pc.wrapping_add(4);
    }

    fn flt_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- (rs1 < rs2) ? 1 : 0;
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.xreg[rd] = if freg_rs1 <= freg_rs2 { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn fsgnjx_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- abs(rs1) * (sgn(rs1) == sgn(rs2)) ? 1 : -1
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        // The sign bit is the XOR of the sign bits of rs1 and rs2.
        let m = if (freg_rs1 < 0.0 && freg_rs2 >= 0.0) || (freg_rs1 >= 0.0 && freg_rs2 < 0.0) {
            -1.0
        } else {
            1.0
        };
        self.freg[rd] = freg_rs1.abs() * m;
        self.pc = self.pc.wrapping_add(4);
    }

    fn feq_s(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- (rs1 == rs2) ? 1 : 0;
        let freg_rs1 = self.freg[rs1];
        let freg_rs2 = self.freg[rs2];
        self.xreg[rd] = if freg_rs1 == freg_rs2 { 1 } else { 0 };
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }
}
