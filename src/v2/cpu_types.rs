use super::{memory::MemoryResult, tobits::Reg, trap_handler::TrapCause};

pub type Address = u32;

pub trait CoreCpu {
    fn get_pc(&self) -> Address;
    fn fetch(&mut self) -> MemoryResult<u32>;
    fn set_next_pc(&mut self, address: Address);

    fn read8(&self, address: Address) -> MemoryResult<u8>;
    fn read16(&self, address: Address) -> MemoryResult<u16>;
    fn read32(&self, address: Address) -> MemoryResult<u32>;

    fn write8(&mut self, address: Address, value: u8) -> MemoryResult<()>;
    fn write16(&mut self, address: Address, value: u16) -> MemoryResult<()>;
    fn write32(&mut self, address: Address, value: u32) -> MemoryResult<()>;

    fn handle_trap(&mut self, cause: TrapCause);
}

pub trait Xreg {
    fn rx(&self, reg: Reg) -> u32;
    fn wx(&mut self, reg: Reg, val: u32);
}

pub trait Freg {
    fn rf(&self, reg: u32) -> f32;
    fn wf(&mut self, reg: u32, val: f32);
}

pub trait DecodeRv32i {
    type Item;

    // Illegal instruction.
    fn illegal(&mut self, ins: u32) -> Self::Item;

    // B-type instructions.
    fn beq(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item;
    fn bne(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item;
    fn blt(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item;
    fn bge(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item;
    fn bltu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item;
    fn bgeu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item;

    // I-type instructions.
    fn lb(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn addi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;
    fn jalr(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;

    // S-type instructions.
    fn sb(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item;
    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item;
    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item;

    // U-type instructions.
    fn auipc(&mut self, rd: Reg, uimm: u32) -> Self::Item;
    fn lui(&mut self, rd: Reg, uimm: u32) -> Self::Item;

    // J-type instructions.
    fn jal(&mut self, rd: Reg, jimm: u32) -> Self::Item;

    // Arithmetic instructions.
    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;

    // Immediate shift instructions.
    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item;
    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item;
    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item;

    // Fence instructions.
    fn fence(&mut self, fm: u32, rd: Reg, rs1: Reg) -> Self::Item;

    // System instructions.
    fn ecall(&mut self) -> Self::Item;
    fn ebreak(&mut self) -> Self::Item;
}

impl<T> DecodeRv32i for T
where
    T: CoreCpu + Xreg,
{
    type Item = ();

    // Illegal instruction.

    fn illegal(&mut self, _ins: u32) {
        self.handle_trap(TrapCause::IllegalInstruction);
    }

    // B-type instructions.

    fn beq(&mut self, rs1: Reg, rs2: Reg, bimm: u32) {
        // pc <- pc + ((rs1 == rs2) ? imm_b : 4)
        if self.rx(rs1) == self.rx(rs2) {
            self.set_next_pc(self.get_pc().wrapping_add(bimm));
        }
    }

    fn bne(&mut self, rs1: Reg, rs2: Reg, bimm: u32) {
        // pc <- pc + ((rs1 != rs2) ? imm_b : 4)
        if self.rx(rs1) != self.rx(rs2) {
            self.set_next_pc(self.get_pc().wrapping_add(bimm));
        }
    }

    fn blt(&mut self, rs1: Reg, rs2: Reg, bimm: u32) {
        // Signed.
        // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
        if (self.rx(rs1) as i32) < (self.rx(rs2) as i32) {
            self.set_next_pc(self.get_pc().wrapping_add(bimm));
        }
    }

    fn bge(&mut self, rs1: Reg, rs2: Reg, bimm: u32) {
        // Signed.
        // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
        if (self.rx(rs1) as i32) >= (self.rx(rs2) as i32) {
            self.set_next_pc(self.get_pc().wrapping_add(bimm));
        }
    }

    fn bltu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) {
        // Unsigned.
        // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
        if self.rx(rs1) < self.rx(rs2) {
            self.set_next_pc(self.get_pc().wrapping_add(bimm));
        }
    }

    fn bgeu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) {
        // Unsigned.
        // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
        if self.rx(rs1) >= self.rx(rs2) {
            self.set_next_pc(self.get_pc().wrapping_add(bimm));
        }
    }

    // I-type instructions.

    fn lb(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- sx(m8(rs1 + imm_i)), pc += 4
        match self.read8(self.rx(rs1).wrapping_add(iimm)) {
            Ok(byte) => {
                self.wx(rd, (((byte as i8) as i16) as i32) as u32); // TODO: this should be a function.
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- sx(m16(rs1 + imm_i)), pc += 4
        match self.read16(self.rx(rs1).wrapping_add(iimm)) {
            Ok(half_word) => {
                self.wx(rd, ((half_word as i16) as i32) as u32); // TODO: this should be a function.
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- sx(m32(rs1 + imm_i)), pc += 4
        match self.read32(self.rx(rs1).wrapping_add(iimm)) {
            Ok(word) => {
                self.wx(rd, word);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- zx(m8(rs1 + imm_i)), pc += 4
        match self.read8(self.rx(rs1).wrapping_add(iimm)) {
            Ok(byte) => {
                self.wx(rd, byte as u32);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- zx(m16(rs1 + imm_i)), pc += 4
        match self.read16(self.rx(rs1).wrapping_add(iimm)) {
            Ok(half_word) => {
                self.wx(rd, half_word as u32);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn addi(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- rs1 + imm_i, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_add(iimm));
    }

    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // Signed.
        // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
        let xreg_rs1 = self.rx(rs1) as i32;
        let iimm = iimm as i32;
        self.wx(rd, if xreg_rs1 < iimm { 1 } else { 0 });
    }

    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // Unsigned.
        // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
        self.wx(rd, if self.rx(rs1) < iimm { 1 } else { 0 });
    }

    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- rs1 ^ imm_i, pc += 4
        self.wx(rd, self.rx(rs1) ^ iimm);
    }

    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- rs1 | imm_i, pc += 4
        self.wx(rd, self.rx(rs1) | iimm);
    }

    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- rs1 & imm_i, pc += 4
        self.wx(rd, self.rx(rs1) & iimm);
    }

    fn jalr(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- pc + 4, pc <- (rs1 + imm_i) & ~1
        let rs1_before = self.rx(rs1); // Because rd and rs1 might be the same register.
        self.wx(rd, self.get_pc().wrapping_add(4));
        self.set_next_pc(rs1_before.wrapping_add(iimm) & !1);
    }

    // S-type instructions.

    fn sb(&mut self, rs1: Reg, rs2: Reg, simm: u32) {
        // m8(rs1 + imm_s) <- rs2[7:0], pc += 4
        if let Err(_) = self.write8(self.rx(rs1).wrapping_add(simm), (self.rx(rs2) & 0xff) as u8) {
            self.handle_trap(TrapCause::StoreAccessFault);
        }
    }

    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) {
        // m16(rs1 + imm_s) <- rs2[15:0], pc += 4
        if let Err(_) = self.write16(
            self.rx(rs1).wrapping_add(simm),
            (self.rx(rs2) & 0xffff) as u16,
        ) {
            self.handle_trap(TrapCause::StoreAccessFault);
        }
    }

    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) {
        // m32(rs1 + imm_s) <- rs2[31:0], pc += 4
        if let Err(_) = self.write32(self.rx(rs1).wrapping_add(simm), self.rx(rs2)) {
            self.handle_trap(TrapCause::StoreAccessFault);
        }
    }

    // U-type instructions.

    fn auipc(&mut self, rd: Reg, uimm: u32) {
        // rd <- pc + imm_u, pc += 4
        self.wx(rd, self.get_pc().wrapping_add(uimm));
    }

    fn lui(&mut self, rd: Reg, uimm: u32) {
        // rd <- imm_u, pc += 4
        self.wx(rd, uimm);
    }

    // J-type instructions.

    fn jal(&mut self, rd: Reg, jimm: u32) {
        // rd <- pc + 4, pc <- pc + imm_j
        self.wx(rd, self.get_pc().wrapping_add(4));
        self.set_next_pc(self.get_pc().wrapping_add(jimm));
    }

    // Arithmetic instructions.

    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 + rs2, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_add(self.rx(rs2)));
    }

    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 - rs2, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_sub(self.rx(rs2)));
    }

    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 << (rs2 % XLEN), pc += 4
        self.wx(rd, self.rx(rs1) << (self.rx(rs2) % 32));
    }

    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // Signed.
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.rx(rs1) as i32;
        let xreg_rs2 = self.rx(rs2) as i32;
        self.wx(rd, if xreg_rs1 < xreg_rs2 { 1 } else { 0 });
    }

    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.rx(rs1);
        let xreg_rs2 = self.rx(rs2);
        self.wx(rd, if xreg_rs1 < xreg_rs2 { 1 } else { 0 });
    }

    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 ^ rs2, pc += 4
        self.wx(rd, self.rx(rs1) ^ self.rx(rs2));
    }

    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        self.wx(rd, self.rx(rs1) >> (self.rx(rs2) % 32));
    }

    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        let xreg_rs1 = self.rx(rs1) as i32;
        let shift = (self.rx(rs2) % 32) as i32;
        self.wx(rd, (xreg_rs1 >> shift) as u32);
    }

    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 | rs2, pc += 4
        self.wx(rd, self.rx(rs1) | self.rx(rs2));
    }

    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 & rs2, pc += 4
        self.wx(rd, self.rx(rs1) & self.rx(rs2));
    }

    // Immediate shift instructions.

    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) {
        self.wx(rd, self.rx(rs1) << shamt);
    }

    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) {
        self.wx(rd, self.rx(rs1) >> shamt);
    }

    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) {
        let xreg_rs = self.rx(rs1) as i32;
        let shamt = shamt as i32;
        self.wx(rd, (xreg_rs >> shamt) as u32);
    }

    // Fence instructions.
    fn fence(&mut self, _fm: u32, _rd: Reg, _rs1: Reg) {}

    // System instructions.
    fn ecall(&mut self) {}
    fn ebreak(&mut self) {}
}

pub trait DecodeRv32m {
    type Item;

    fn mul(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn mulh(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn mulhsu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn mulhu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn div(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn divu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn rem(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn remu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
}

impl<T> DecodeRv32m for T
where
    T: CoreCpu + Xreg,
{
    type Item = ();

    fn mul(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        // rd <- rs1 * rs2, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_mul(self.rx(rs2)));
    }

    fn mulh(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let xreg_rs1 = (self.rx(rs1) as i32) as i64;
        let xreg_rs2 = (self.rx(rs2) as i32) as i64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.wx(rd, t as u32);
    }

    fn mulhsu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let xreg_rs1 = (self.rx(rs1) as i32) as i64;
        let xreg_rs2 = (self.rx(rs2) as u64) as i64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.wx(rd, t as u32);
    }

    fn mulhu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let xreg_rs1 = self.rx(rs1) as u64;
        let xreg_rs2 = self.rx(rs2) as u64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.wx(rd, t as u32);
    }

    fn div(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let dividend = self.rx(rs1) as i32;
        let divisor = self.rx(rs2) as i32;
        // Check for signed division overflow.
        if ((dividend as u32) != 0x80000000) || divisor != -1 {
            self.wx(
                rd,
                if divisor != 0 {
                    (dividend.wrapping_div(divisor)) as u32
                } else {
                    u32::MAX // -1.
                },
            )
        } else {
            // Signed division overflow occurred.
            self.wx(rd, dividend as u32);
        }
    }

    fn divu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let dividend = self.rx(rs1);
        let divisor = self.rx(rs2);
        self.wx(
            rd,
            if divisor != 0 {
                dividend.wrapping_div(divisor)
            } else {
                u32::MAX // -1.
            },
        );
    }

    fn rem(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let dividend = self.rx(rs1) as i32;
        let divisor = self.rx(rs2) as i32;
        // Check for signed division overflow.
        if ((dividend as u32) != 0x80000000) || divisor != -1 {
            self.wx(
                rd,
                if divisor != 0 {
                    (dividend % divisor) as u32
                } else {
                    dividend as u32
                },
            )
        } else {
            // Signed division overflow occurred.
            self.wx(rd, 0);
        }
    }

    fn remu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {
        let dividend = self.rx(rs1);
        let divisor = self.rx(rs2);
        self.wx(
            rd,
            if divisor != 0 {
                dividend % divisor
            } else {
                dividend
            },
        );
    }
}
