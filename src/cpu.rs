use crate::{
    Bimm12Rs1Rs2, Decoder, Imm12RdRs1, Imm12Rs1Rs2, Imm20Rd, Jimm20Rd, NoArgs, RdFmPredRdRs1Succ,
    RdRs1, RdRs1Rm, RdRs1Rs2, RdRs1Rs2Rm, RdRs1Rs2Rs3Rm, RdRs1Shamtw, Trap,
};

type Address = u32;

pub enum BusCode {
    LoadAccessFault,
    StoreAccessFault,
}

type MemoryResult<T> = Result<T, BusCode>;

pub trait Mem {
    fn read8(&self, address: Address) -> MemoryResult<u8>;
    fn read16(&self, address: Address) -> MemoryResult<u16>;
    fn read32(&self, address: Address) -> MemoryResult<u32>;
    fn write8(&mut self, address: Address, byte: u8) -> MemoryResult<()>;
    fn write16(&mut self, address: Address, half_word: u16) -> MemoryResult<()>;
    fn write32(&mut self, address: Address, word: u32) -> MemoryResult<()>;
}

pub struct Cpu<T: Mem> {
    pc: u32,         // The program counter.
    xreg: [u32; 32], // Regular registerx, x0-x31.
    freg: [f32; 32], // Floating point registers, f0-f31.
    mem: T,
}

impl<T: Mem> Cpu<T> {
    pub fn handle_trap(&self) {
        println!("TRAP");
    }

    pub fn handle_ecall(&self) {
        println!("ECALL");
    }

    pub fn handle_ebreak(&self) {
        println!("EBREAK");
    }
}

struct BasicMem;

impl Mem for BasicMem {
    fn read8(&self, address: Address) -> MemoryResult<u8> {
        Ok(0)
    }

    fn read16(&self, address: Address) -> MemoryResult<u16> {
        Ok(0)
    }

    fn read32(&self, address: Address) -> MemoryResult<u32> {
        Ok(0)
    }

    fn write8(&mut self, address: Address, byte: u8) -> MemoryResult<()> {
        Ok(())
    }

    fn write16(&mut self, address: Address, half_word: u16) -> MemoryResult<()> {
        Ok(())
    }

    fn write32(&mut self, address: Address, word: u32) -> MemoryResult<()> {
        Ok(())
    }
}

impl Cpu<BasicMem> {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            xreg: Default::default(),
            freg: Default::default(),
            mem: BasicMem,
        }
    }
}

impl<T: Mem> Decoder for Cpu<T> {
    type Item = ();

    fn trap(&mut self, instruction: Trap, machine_code: u32) -> Self::Item {
        self.handle_trap();
    }

    fn b_type(&mut self, instruction: Bimm12Rs1Rs2, bimm: i32, rs1: u8, rs2: u8) -> Self::Item {
        let bimm = bimm as u32;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            Bimm12Rs1Rs2::Beq => {
                // pc <- pc + ((rs1 == rs2) ? imm_b : 4)
                self.pc += if self.xreg[rs1] == self.xreg[rs2] {
                    bimm
                } else {
                    4
                };
            }
            Bimm12Rs1Rs2::Bne => {
                // pc <- pc + ((rs1 != rs2) ? imm_b : 4)
                self.pc += if self.xreg[rs1] != self.xreg[rs2] {
                    bimm
                } else {
                    4
                };
            }
            Bimm12Rs1Rs2::Blt => {
                // Signed.
                // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
                self.pc += if (self.xreg[rs1] as i32) < (self.xreg[rs2] as i32) {
                    bimm
                } else {
                    4
                };
            }
            Bimm12Rs1Rs2::Bge => {
                // Signed.
                // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
                self.pc += if (self.xreg[rs1] as i32) >= (self.xreg[rs2] as i32) {
                    bimm
                } else {
                    4
                };
            }
            Bimm12Rs1Rs2::Bltu => {
                // Unsigned.
                // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
                self.pc += if self.xreg[rs1] < self.xreg[rs2] {
                    bimm
                } else {
                    4
                };
            }
            Bimm12Rs1Rs2::Bgeu => {
                // Unsigned.
                // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
                self.pc += if self.xreg[rs1] >= self.xreg[rs2] {
                    bimm
                } else {
                    4
                };
            }
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
            Imm12RdRs1::Lb => {
                // rd <- sx(m8(rs1 + imm_i)), pc += 4
                match self.mem.read8(self.xreg[rs1] + iimm) {
                    Ok(byte) => {
                        self.xreg[rd] = (((byte as i8) as i16) as i32) as u32; // TODO: this should be a function.
                        self.pc += 4;
                        self.xreg[0] = 0;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a load access fault.
                    }
                }
            }
            Imm12RdRs1::Lh => {
                // rd <- sx(m16(rs1 + imm_i)), pc += 4
                match self.mem.read16(self.xreg[rs1] + iimm) {
                    Ok(half_word) => {
                        self.xreg[rd] = ((half_word as i16) as i32) as u32; // TODO: this should be a function.
                        self.pc += 4;
                        self.xreg[0] = 0;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a load access fault.
                    }
                }
            }
            Imm12RdRs1::Lw => {
                // rd <- sx(m32(rs1 + imm_i)), pc += 4
                match self.mem.read32(self.xreg[rs1] + iimm) {
                    Ok(word) => {
                        self.xreg[rd] = word;
                        self.pc += 4;
                        self.xreg[0] = 0;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a load access fault.
                    }
                }
            }
            Imm12RdRs1::Lbu => {
                // rd <- zx(m8(rs1 + imm_i)), pc += 4
                match self.mem.read8(self.xreg[rs1] + iimm) {
                    Ok(byte) => {
                        self.xreg[rd] = byte as u32;
                        self.pc += 4;
                        self.xreg[0] = 0;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a load access fault.
                    }
                }
            }
            Imm12RdRs1::Lhu => {
                // rd <- zx(m16(rs1 + imm_i)), pc += 4
                match self.mem.read16(self.xreg[rs1] + iimm) {
                    Ok(half_word) => {
                        self.xreg[rd] = half_word as u32;
                        self.pc += 4;
                        self.xreg[0] = 0;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a load access fault.
                    }
                }
            }
            Imm12RdRs1::Flw => {
                // rd <- f32(rs1 + imm_i)
                match self.mem.read32(self.xreg[rs1] + iimm) {
                    Ok(word) => {
                        self.freg[rd] = f32::from_bits(word);
                        self.pc += 4;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a load access fault.
                    }
                }
            }
            Imm12RdRs1::Addi => {
                // rd <- rs1 + imm_i, pc += 4
                self.xreg[rd] = self.xreg[rs1] + iimm;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm12RdRs1::Slti => {
                // Signed.
                // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
                let xreg_rs1 = self.xreg[rs1] as i32;
                let iimm = iimm as i32;
                self.xreg[rd] = if xreg_rs1 < iimm { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm12RdRs1::Sltiu => {
                // Unsigned.
                // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
                self.xreg[rd] = if self.xreg[rs1] < iimm { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm12RdRs1::Xori => {
                // rd <- rs1 ^ imm_i, pc += 4
                self.xreg[rd] = self.xreg[rs1] ^ iimm;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm12RdRs1::Ori => {
                // rd <- rs1 | imm_i, pc += 4
                self.xreg[rd] = self.xreg[rs1] | iimm;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm12RdRs1::Andi => {
                // rd <- rs1 & imm_i, pc += 4
                self.xreg[rd] = self.xreg[rs1] & iimm;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm12RdRs1::Jalr => {
                // rd <- pc + 4, pc <- (rs1 + imm_i) & ~1
                let rs1_before = self.xreg[rs1]; // Because rd and rs1 might be the same register.
                self.xreg[rd] = self.pc + 4;
                self.pc = (rs1_before + iimm) & !1;
                self.xreg[0] = 0;
            }
        }
    }

    fn s_type(&mut self, instruction: Imm12Rs1Rs2, simm: i32, rs1: u8, rs2: u8) -> Self::Item {
        let simm = simm as u32;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            Imm12Rs1Rs2::Sb => {
                // m8(rs1 + imm_s) <- rs2[7:0], pc += 4
                match self
                    .mem
                    .write8(self.xreg[rs1] + simm, (self.xreg[rs2] & 0xff) as u8)
                {
                    Ok(_) => {
                        self.pc += 4;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a store access fault.
                    }
                }
            }
            Imm12Rs1Rs2::Sh => {
                // m16(rs1 + imm_s) <- rs2[15:0], pc += 4
                match self
                    .mem
                    .write16(self.xreg[rs1] + simm, (self.xreg[rs2] & 0xffff) as u16)
                {
                    Ok(_) => {
                        self.pc += 4;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a store access fault.
                    }
                }
            }
            Imm12Rs1Rs2::Sw => {
                // m32(rs1 + imm_s) <- rs2[31:0], pc += 4
                match self.mem.write32(self.xreg[rs1] + simm, self.xreg[rs2]) {
                    Ok(_) => {
                        self.pc += 4;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a store access fault.
                    }
                }
            }
            Imm12Rs1Rs2::Fsw => {
                // f32(rs1 + imm_s) = rs2
                let data = f32::to_bits(self.freg[rs2]);
                match self.mem.write32(self.xreg[rs1] + simm, data) {
                    Ok(_) => {
                        self.pc += 4;
                    }
                    Err(_) => {
                        self.handle_trap(); // TODO: it's a store access fault.
                    }
                }
            }
        }
    }

    fn u_type(&mut self, instruction: Imm20Rd, uimm: i32, rd: u8) -> Self::Item {
        let uimm = uimm as u32;
        let rd = rd as usize;
        match instruction {
            Imm20Rd::Auipc => {
                // rd <- pc + imm_u, pc += 4
                self.xreg[rd] = self.pc + uimm;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            Imm20Rd::Lui => {
                // rd <- imm_u, pc += 4
                self.xreg[rd] = uimm;
                self.pc += 4;
                self.xreg[0] = 0;
            }
        }
    }

    fn j_type(&mut self, instruction: Jimm20Rd, jimm: i32, rd: u8) -> Self::Item {
        let jimm = jimm as u32;
        let rd = rd as usize;
        match instruction {
            Jimm20Rd::Jal => {
                // rd <- pc + 4, pc <- pc + imm_j
                self.xreg[rd] = self.pc + 4;
                self.pc += jimm;
                self.xreg[0] = 0;
            }
        }
    }

    fn no_args(&mut self, instruction: NoArgs) -> Self::Item {
        match instruction {
            NoArgs::Ecall => self.handle_ecall(),
            NoArgs::Ebreak => self.handle_ebreak(),
        }
    }

    fn rd_rm_rs1(&mut self, instruction: RdRs1Rm, rd: u8, rm: u8, rs1: u8) -> Self::Item {
        let rd = rd as usize;
        let rm = rm as usize;
        let rs1 = rs1 as usize;
        match instruction {
            RdRs1Rm::FsqrtS => {
                // rd <- sqrt(rs1)
                let f = self.freg[rs1];
                self.freg[rd] = f32::sqrt(f);
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rm::FcvtWS => {
                // rd <- int32_t(rs1)
                let i = self.freg[rs1] as i32;
                self.xreg[rd] = i as u32;
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rm::FcvtWuS => {
                // rd <- uint32_t(rs1)
                let i = self.freg[rs1] as u32;
                self.xreg[rd] = i;
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rm::FcvtSW => {
                // rd <- float(int32_t((rs1))
                let i = self.xreg[rs1] as i32;
                self.freg[rd] = i as f32;
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rm::FcvtSWu => {
                // rd <- float(rs1)
                self.freg[rd] = self.xreg[rs1] as f32;
                self.pc += 4;
                // TODO: handle rounding modes.
            }
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
            RdRs1Rs2Rm::FaddS => {
                // rd <- rs1 + rs2
                self.freg[rd] = self.freg[rs1] + self.freg[rs2];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rs2Rm::FsubS => {
                // rd <- rs1 - rs2
                self.freg[rd] = self.freg[rs1] - self.freg[rs2];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rs2Rm::FmulS => {
                // rd <- rs1 * rs2
                self.freg[rd] = self.freg[rs1] * self.freg[rs2];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rs2Rm::FdivS => {
                // rd <- rs1 / rs2
                self.freg[rd] = self.freg[rs1] / self.freg[rs2];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
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
            RdRs1Rs2Rs3Rm::FmaddS => {
                // rd <- (rs1 * rs2) + rs3
                self.freg[rd] = (self.freg[rs1] * self.freg[rs2]) + self.freg[rs3];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rs2Rs3Rm::FmsubS => {
                // rd <- (rs1 * rs2) - rs3
                self.freg[rd] = (self.freg[rs1] * self.freg[rs2]) - self.freg[rs3];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rs2Rs3Rm::FnmsubS => {
                // rd <- -(rs1 * rs2) + rs3
                self.freg[rd] = -(self.freg[rs1] * self.freg[rs2]) + self.freg[rs3];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
            RdRs1Rs2Rs3Rm::FnmaddS => {
                // rd <- -(rs1 * rs2) - rs3
                self.freg[rd] = -(self.freg[rs1] * self.freg[rs2]) - self.freg[rs3];
                self.pc += 4;
                // TODO: handle rounding modes.
            }
        }
    }

    fn rd_rs1(&mut self, instruction: RdRs1, rd: u8, rs1: u8) -> Self::Item {
        let rd = rd as usize;
        let rs1 = rs1 as usize;
        match instruction {
            RdRs1::FmvXW => {
                // bits(rd) <- bits(rs1)
                self.xreg[rd] = f32::to_bits(self.freg[rs1]);
                self.pc += 4;
            }
            RdRs1::FmvWX => {
                // bits(rd) <- bits(rs1)
                self.freg[rd] = f32::from_bits(self.xreg[rs1]);
            }
            RdRs1::FclassS => {
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
                self.pc += 4;
                self.xreg[0] = 0;
            }
        }
    }

    fn rd_rs1_rs2(&mut self, instruction: RdRs1Rs2, rd: u8, rs1: u8, rs2: u8) -> Self::Item {
        let rd = rd as usize;
        let rs1 = rs1 as usize;
        let rs2 = rs2 as usize;
        match instruction {
            RdRs1Rs2::Add => {
                // rd <- rs1 + rs2, pc += 4
                self.xreg[rd] = self.xreg[rs1] + self.xreg[rs2];
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Mul => {
                // rd <- rs1 * rs2, pc += 4
                self.xreg[rd] = self.xreg[rs1] * self.xreg[rs2];
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Sub => {
                // rd <- rs1 - rs2, pc += 4
                self.xreg[rd] = self.xreg[rs1] - self.xreg[rs2];
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Sll => {
                // rd <- rs1 << (rs2 % XLEN), pc += 4
                self.xreg[rd] = self.xreg[rs1] << (self.xreg[rs2] % 32);
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Mulh => {
                let xreg_rs1 = (self.xreg[rs1] as i32) as i64;
                let xreg_rs2 = (self.xreg[rs2] as i32) as i64;
                let t = (xreg_rs1 * xreg_rs2) >> 32;
                self.xreg[rd] = t as u32;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Slt => {
                // Signed.
                // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
                let xreg_rs1 = self.xreg[rs1] as i32;
                let xreg_rs2 = self.xreg[rs2] as i32;
                self.xreg[rd] = if xreg_rs1 < xreg_rs2 { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Mulhsu => {
                let xreg_rs1 = (self.xreg[rs1] as i32) as i64;
                let xreg_rs2 = (self.xreg[rs2] as u64) as i64;
                let t = (xreg_rs1 * xreg_rs2) >> 32;
                self.xreg[rd] = t as u32;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Sltu => {
                // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
                let xreg_rs1 = self.xreg[rs1];
                let xreg_rs2 = self.xreg[rs2];
                self.xreg[rd] = if xreg_rs1 < xreg_rs2 { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Mulhu => {
                let xreg_rs1 = self.xreg[rs1] as u64;
                let xreg_rs2 = self.xreg[rs2] as u64;
                let t = (xreg_rs1 * xreg_rs2) >> 32;
                self.xreg[rd] = t as u32;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Xor => {
                // rd <- rs1 ^ rs2, pc += 4
                self.xreg[rd] = self.xreg[rs1] ^ self.xreg[rs2];
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Div => {
                let dividend = self.xreg[rs1] as i32;
                let divisor = self.xreg[rs2] as i32;
                // Check for signed division overflow.
                if ((dividend as u32) != 0x80000000) || divisor != -1 {
                    self.xreg[rd] = if divisor != 0 {
                        (dividend / divisor) as u32
                    } else {
                        u32::MAX // -1.
                    }
                } else {
                    // Signed division overflow occurred.
                    self.xreg[rd] = dividend as u32;
                }
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Srl => {
                // rd <- rs1 >> (rs2 % XLEN), pc += 4
                self.xreg[rd] = self.xreg[rs1] >> (self.xreg[rs2] % 32);
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Divu => {
                let dividend = self.xreg[rs1];
                let divisor = self.xreg[rs2];
                self.xreg[rd] = if divisor != 0 {
                    dividend / divisor
                } else {
                    u32::MAX // -1.
                };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Sra => {
                // rd <- rs1 >> (rs2 % XLEN), pc += 4
                let xreg_rs1 = self.xreg[rs1] as i32;
                let shift = (self.xreg[rs2] % 32) as i32;
                self.xreg[rd] = (xreg_rs1 >> shift) as u32;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Or => {
                // rd <- rs1 | rs2, pc += 4
                self.xreg[rd] = self.xreg[rs1] | self.xreg[rs2];
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Rem => {
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
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::And => {
                // rd <- rs1 & rs2, pc += 4
                self.xreg[rd] = self.xreg[rs1] & self.xreg[rs2];
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::Remu => {
                let dividend = self.xreg[rs1];
                let divisor = self.xreg[rs2];
                self.xreg[rd] = if divisor != 0 {
                    dividend % divisor
                } else {
                    dividend
                };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::FsgnjS => {
                // rd <- abs(rs1) * sgn(rs2)
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.freg[rd] = freg_rs1.abs() * if freg_rs2 < 0.0 { -1.0 } else { 1.0 };
                self.pc += 4;
            }
            RdRs1Rs2::FminS => {
                // rd <- min(rs1, rs2)
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.freg[rd] = freg_rs1.min(freg_rs2);
                self.pc += 4;
            }
            RdRs1Rs2::FleS => {
                // rd <- (rs1 <= rs2) ? 1 : 0;
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.xreg[rd] = if freg_rs1 < freg_rs2 { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::FsgnjnS => {
                // rd <- abs(rs1) * -sgn(rs2)
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.freg[rd] = freg_rs1.abs() * if freg_rs2 < 0.0 { 1.0 } else { -1.0 };
                self.pc += 4;
            }
            RdRs1Rs2::FmaxS => {
                // rd <- max(rs1, rs2)
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.freg[rd] = freg_rs1.max(freg_rs2);
                self.pc += 4;
            }
            RdRs1Rs2::FltS => {
                // rd <- (rs1 < rs2) ? 1 : 0;
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.xreg[rd] = if freg_rs1 <= freg_rs2 { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Rs2::FsgnjxS => {
                // rd <- abs(rs1) * (sgn(rs1) == sgn(rs2)) ? 1 : -1
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                // The sign bit is the XOR of the sign bits of rs1 and rs2.
                let m =
                    if (freg_rs1 < 0.0 && freg_rs2 >= 0.0) || (freg_rs1 >= 0.0 && freg_rs2 < 0.0) {
                        -1.0
                    } else {
                        1.0
                    };
                self.freg[rd] = freg_rs1.abs() * m;
                self.pc += 4;
            }
            RdRs1Rs2::FeqS => {
                // rd <- (rs1 == rs2) ? 1 : 0;
                let freg_rs1 = self.freg[rs1];
                let freg_rs2 = self.freg[rs2];
                self.xreg[rd] = if freg_rs1 == freg_rs2 { 1 } else { 0 };
                self.pc += 4;
                self.xreg[0] = 0;
            }
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
            RdRs1Shamtw::Slli => {
                self.xreg[rd] = self.xreg[rs1] << shamt;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Shamtw::Srli => {
                self.xreg[rd] = self.xreg[rs1] >> shamt;
                self.pc += 4;
                self.xreg[0] = 0;
            }
            RdRs1Shamtw::Srai => {
                let xreg_rs = self.xreg[rs1] as i32;
                let shamt = shamt as i32;
                self.xreg[rd] = (xreg_rs >> shamt) as u32;
                self.pc += 4;
                self.xreg[0] = 0;
            }
        }
    }
}
