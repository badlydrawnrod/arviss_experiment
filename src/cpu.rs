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
        todo!()
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

    fn fence(&mut self, instruction: RdFmPredRdRs1Succ, fm: u8, rd: u8, rs1: u8) -> Self::Item {
        todo!()
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
                        todo!() // TODO: take a trap for a load access fault.
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
                        todo!() // TODO: take a trap for a load access fault.
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
                        todo!() // TODO: take a trap for a load access fault.
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
                        todo!() // TODO: take a trap for a load access fault.
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
                        todo!() // TODO: take a trap for a load access fault.
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
                        todo!() // TODO: take a trap for a load access fault.
                    }
                }
            }
            Imm12RdRs1::FenceI => {
                // nop
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
        todo!()
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
        todo!()
    }

    fn rd_rm_rs1(&mut self, instruction: RdRs1Rm, rd: u8, rm: u8, rs1: u8) -> Self::Item {
        todo!()
    }

    fn rd_rm_rs1_rs2(
        &mut self,
        instruction: RdRs1Rs2Rm,
        rd: u8,
        rm: u8,
        rs1: u8,
        rs2: u8,
    ) -> Self::Item {
        todo!()
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
        todo!()
    }

    fn rd_rs1(&mut self, instruction: RdRs1, rd: u8, rs1: u8) -> Self::Item {
        todo!()
    }

    fn rd_rs1_rs2(&mut self, instruction: RdRs1Rs2, rd: u8, rs1: u8, rs2: u8) -> Self::Item {
        todo!()
    }

    fn rd_rs1_shamtw(
        &mut self,
        instruction: RdRs1Shamtw,
        rd: u8,
        rs1: u8,
        shamt: u8,
    ) -> Self::Item {
        todo!()
    }
}
