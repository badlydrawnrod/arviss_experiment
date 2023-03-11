use super::{
    memory::{Mem, MemoryResult},
    trap_handler::{TrapCause, TrapHandler},
};

pub type Address = u32;

pub trait CoreCpu {
    type Item;

    fn rpc(&self) -> Address;
    fn wpc(&mut self, address: Address);

    fn read8(&self, address: Address) -> MemoryResult<u8>;
    fn read16(&self, address: Address) -> MemoryResult<u8>;
    fn read32(&self, address: Address) -> MemoryResult<u8>;

    fn write8(&mut self, address: Address, value: u8) -> MemoryResult<()>;
    fn write16(&mut self, address: Address, value: u16) -> MemoryResult<()>;
    fn write32(&mut self, address: Address, value: u32) -> MemoryResult<()>;

    fn handle_trap(&mut self, cause: TrapCause);
}

pub trait Xreg {
    fn rx(&self, reg: usize) -> u32;
    fn wx(&mut self, reg: usize, val: u32);
}

pub trait Freg {
    fn rf(&self, reg: usize) -> f32;
    fn wf(&mut self, reg: usize, val: f32);
}

pub trait Rv32i: CoreCpu + Xreg {
    // Illegal instruction.
    fn illegal(&mut self, ins: u32) -> Self::Item;

    // B-type instructions.
    fn beq(&mut self, rs1: u32, rs2: u32, bimm: u32) -> Self::Item;
    fn bne(&mut self, rs1: u32, rs2: u32, bimm: u32) -> Self::Item;
    fn blt(&mut self, rs1: u32, rs2: u32, bimm: u32) -> Self::Item;
    fn bge(&mut self, rs1: u32, rs2: u32, bimm: u32) -> Self::Item;
    fn bltu(&mut self, rs1: u32, rs2: u32, bimm: u32) -> Self::Item;
    fn bgeu(&mut self, rs1: u32, rs2: u32, bimm: u32) -> Self::Item;

    // I-type instructions.
    fn lb(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn lh(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn lw(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn lbu(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn lhu(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn addi(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn slti(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn sltiu(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn xori(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn ori(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn andi(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;
    fn jalr(&mut self, rd: u32, rs1: u32, iimm: u32) -> Self::Item;

    // S-type instructions.
    fn sb(&mut self, rs1: u32, rs2: u32, simm: u32) -> Self::Item;
    fn sh(&mut self, rs1: u32, rs2: u32, simm: u32) -> Self::Item;
    fn sw(&mut self, rs1: u32, rs2: u32, simm: u32) -> Self::Item;

    // U-type instructions.
    fn auipc(&mut self, rd: u32, uimm: u32) -> Self::Item;
    fn lui(&mut self, rd: u32, uimm: u32) -> Self::Item;

    // J-type instructions.
    fn jal(&mut self, rd: u32, jimm: u32) -> Self::Item;

    // Register arithmetic instructions.
    fn add(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn sub(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn sll(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn slt(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn sltu(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn xor(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn srl(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn sra(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn or(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn and(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;

    // Immediate shift instructions.
    fn slli(&mut self, rd: u32, rs1: u32, shamt: u32) -> Self::Item;
    fn srli(&mut self, rd: u32, rs1: u32, shamt: u32) -> Self::Item;
    fn srai(&mut self, rd: u32, rs1: u32, shamt: u32) -> Self::Item;

    // Fence instructions.
    fn fence(&mut self, fm: u32, rd: u32, rs1: u32) -> Self::Item;

    // System instructions.
    fn ecall(&mut self) -> Self::Item;
    fn ebreak(&mut self) -> Self::Item;
}

pub trait Rv32m: Rv32i {
    fn mul(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn mulh(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn mulhsu(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn mulhu(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn div(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn divu(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn rem(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
    fn remu(&mut self, rd: u32, rs1: u32, rs2: u32) -> Self::Item;
}

impl<T, U> Rv32i
where
    T: Mem,
    U: TrapHandler<Item = ()>,
{
    // Illegal instruction.

    fn illegal(&mut self, ins: u32) {}

    // B-type instructions.

    fn beq(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // pc <- pc + ((rs1 == rs2) ? imm_b : 4)
        let offset = if self.rx(rs1) == self.rx(rs2) {
            bimm
        } else {
            4
        };
        self.wpc(self.rpc().wrapping_add(offset));
    }

    fn bne(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // pc <- pc + ((rs1 != rs2) ? imm_b : 4)
        let offset = if self.rx(rs1) != self.rx(rs2) {
            bimm
        } else {
            4
        };
        self.wpc(self.rpc().wrapping_add(offset));
    }

    fn blt(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Signed.
        // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
        let offset = if (self.rx(rs1) as i32) < (self.rx(rs2) as i32) {
            bimm
        } else {
            4
        };
        self.wpc(self.rpc().wrapping_add(offset));
    }

    fn bge(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Signed.
        // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
        let offset = if (self.rx(rs1) as i32) >= (self.rx(rs2) as i32) {
            bimm
        } else {
            4
        };
        self.wpc(self.rpc().wrapping_add(offset));
    }

    fn bltu(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Unsigned.
        // pc <- pc + ((rs1 < rs2) ? imm_b : 4)
        let offset = if self.rx(rs1) < self.rx(rs2) { bimm } else { 4 };
        self.wpc(self.rpc().wrapping_add(offset));
    }

    fn bgeu(&mut self, rs1: usize, rs2: usize, bimm: u32) {
        // Unsigned.
        // pc <- pc + ((rs1 >= rs2) ? imm_b : 4)
        let offset = if self.rx(rs1) >= self.rx(rs2) {
            bimm
        } else {
            4
        };
        self.wpc(self.rpc().wrapping_add(offset));
    }

    // I-type instructions.

    fn lb(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m8(rs1 + imm_i)), pc += 4
        match self.read8(self.rx(rs1).wrapping_add(iimm)) {
            Ok(byte) => {
                self.wx(rd, (((byte as i8) as i16) as i32) as u32); // TODO: this should be a function.
                self.wpc(self.rpc().wrapping_add(4));
                self.wx(0, 0);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lh(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m16(rs1 + imm_i)), pc += 4
        match self.read16(self.rx(rs1).wrapping_add(iimm)) {
            Ok(half_word) => {
                self.wx(rd, ((half_word as i16) as i32) as u32); // TODO: this should be a function.
                self.wpc(self.rpc().wrapping_add(4));
                self.wx(0, 0);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lw(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- sx(m32(rs1 + imm_i)), pc += 4
        match self.read32(self.rx(rs1).wrapping_add(iimm)) {
            Ok(word) => {
                self.wx(rd, word);
                self.wpc(self.rpc().wrapping_add(4));
                self.wx(0, 0);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lbu(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- zx(m8(rs1 + imm_i)), pc += 4
        match self.read8(self.rx(rs1).wrapping_add(iimm)) {
            Ok(byte) => {
                self.wx(rd, byte as u32);
                self.wpc(self.rpc().wrapping_add(4));
                self.wx(0, 0);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn lhu(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- zx(m16(rs1 + imm_i)), pc += 4
        match self.read16(self.rx(rs1).wrapping_add(iimm)) {
            Ok(half_word) => {
                self.wx(rd, half_word as u32);
                self.wpc(self.rpc().wrapping_add(4));
                self.wx(0, 0);
            }
            Err(_) => {
                self.handle_trap(TrapCause::LoadAccessFault);
            }
        }
    }

    fn addi(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 + imm_i, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_add(iimm));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn slti(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // Signed.
        // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
        let xreg_rs1 = self.rx(rs1) as i32;
        let iimm = iimm as i32;
        self.wx(rd, if xreg_rs1 < iimm { 1 } else { 0 });
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn sltiu(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // Unsigned.
        // rd <- (rs1 < imm_i) ? 1 : 0, pc += 4
        self.wx(rd, if self.rx(rs1) < iimm { 1 } else { 0 });
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn xori(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 ^ imm_i, pc += 4
        self.wx(rd, self.rx(rs1) ^ iimm);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn ori(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 | imm_i, pc += 4
        self.wx(rd, self.rx(rs1) | iimm);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn andi(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- rs1 & imm_i, pc += 4
        self.wx(rd, self.rx(rs1) & iimm);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn jalr(&mut self, rd: usize, rs1: usize, iimm: u32) {
        // rd <- pc + 4, pc <- (rs1 + imm_i) & ~1
        let rs1_before = self.rx(rs1); // Because rd and rs1 might be the same register.
        self.wx(rd, self.rpc().wrapping_add(4));
        self.wpc(rs1_before.wrapping_add(iimm) & !1);
        self.wx(0, 0);
    }

    // S-type instructions.

    fn sb(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // m8(rs1 + imm_s) <- rs2[7:0], pc += 4
        match self.write8(self.rx(rs1).wrapping_add(simm), (self.rx(rs2) & 0xff) as u8) {
            Ok(_) => {
                self.wpc(self.rpc().wrapping_add(4));
            }
            Err(_) => {
                self.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    fn sh(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // m16(rs1 + imm_s) <- rs2[15:0], pc += 4
        match self.write16(
            self.rx(rs1).wrapping_add(simm),
            (self.rx(rs2) & 0xffff) as u16,
        ) {
            Ok(_) => {
                self.wpc(self.rpc().wrapping_add(4));
            }
            Err(_) => {
                self.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    fn sw(&mut self, rs1: usize, rs2: usize, simm: u32) {
        // m32(rs1 + imm_s) <- rs2[31:0], pc += 4
        match self.write32(self.rx(rs1).wrapping_add(simm), self.rx(rs2)) {
            Ok(_) => {
                self.wpc(self.rpc().wrapping_add(4));
            }
            Err(_) => {
                self.handle_trap(TrapCause::StoreAccessFault);
            }
        }
    }

    // U-type instructions.

    fn auipc(&mut self, rd: usize, uimm: u32) {
        // rd <- pc + imm_u, pc += 4
        self.wx(rd, self.rpc().wrapping_add(uimm));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn lui(&mut self, rd: usize, uimm: u32) {
        // rd <- imm_u, pc += 4
        self.wx(rd, uimm);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    // J-type instructions.

    fn jal(&mut self, rd: usize, jimm: u32) {
        // rd <- pc + 4, pc <- pc + imm_j
        self.wx(rd, self.rpc().wrapping_add(4));
        self.wpc(self.rpc().wrapping_add(jimm));
        self.wx(0, 0);
    }

    // Arithmetic instructions.

    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 + rs2, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_add(self.rx(rs2)));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 - rs2, pc += 4
        self.wx(rd, self.rx(rs1).wrapping_sub(self.rx(rs2)));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 << (rs2 % XLEN), pc += 4
        self.wx(rd, self.rx(rs1) << (self.rx(rs2) % 32));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // Signed.
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.rx(rs1) as i32;
        let xreg_rs2 = self.rx(rs2) as i32;
        self.wx(rd, if xreg_rs1 < xreg_rs2 { 1 } else { 0 });
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.rx(rs1);
        let xreg_rs2 = self.rx(rs2);
        self.wx(rd, if xreg_rs1 < xreg_rs2 { 1 } else { 0 });
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 ^ rs2, pc += 4
        self.wx(rd, self.rx(rs1) ^ self.rx(rs2));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        self.wx(rd, self.rx(rs1) >> (self.rx(rs2) % 32));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        let xreg_rs1 = self.rx(rs1) as i32;
        let shift = (self.rx(rs2) % 32) as i32;
        self.wx(rd, (xreg_rs1 >> shift) as u32);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 | rs2, pc += 4
        self.wx(rd, self.rx(rs1) | self.rx(rs2));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 & rs2, pc += 4
        self.wx(rd, self.rx(rs1) & self.rx(rs2));
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    // Immediate shift instructions.

    fn slli(&mut self, rd: usize, rs1: usize, shamt: u32) {
        self.wx(rd, self.rx(rs1) << shamt);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn srli(&mut self, rd: usize, rs1: usize, shamt: u32) {
        self.wx(rd, self.rx(rs1) >> shamt);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    fn srai(&mut self, rd: usize, rs1: usize, shamt: u32) {
        let xreg_rs = self.rx(rs1) as i32;
        let shamt = shamt as i32;
        self.wx(rd, (xreg_rs >> shamt) as u32);
        self.wpc(self.rpc().wrapping_add(4));
        self.wx(0, 0);
    }

    // Fence instructions.
    fn fence(&mut self, fm: usize, rd: usize, rs1: usize) {}

    // System instructions.
    fn ecall(&mut self) {}
    fn ebreak(&mut self) {}
}

// impl<T, U> Rv32m for Cpu<T, U>
// where
//     T: Mem,
//     U: TrapHandler<Item = ()>,
// {
//     fn mul(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         // rd <- rs1 * rs2, pc += 4
//         self.wx(rd, self.rx(rs1).wrapping_mul(self.rx(rs2)));
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn mulh(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let xreg_rs1 = (self.rx(rs1) as i32) as i64;
//         let xreg_rs2 = (self.rx(rs2) as i32) as i64;
//         let t = (xreg_rs1 * xreg_rs2) >> 32;
//         self.wx(rd, t as u32);
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn mulhsu(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let xreg_rs1 = (self.rx(rs1) as i32) as i64;
//         let xreg_rs2 = (self.rx(rs2) as u64) as i64;
//         let t = (xreg_rs1 * xreg_rs2) >> 32;
//         self.wx(rd, t as u32);
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn mulhu(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let xreg_rs1 = self.rx(rs1) as u64;
//         let xreg_rs2 = self.rx(rs2) as u64;
//         let t = (xreg_rs1 * xreg_rs2) >> 32;
//         self.wx(rd, t as u32);
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn div(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let dividend = self.rx(rs1) as i32;
//         let divisor = self.rx(rs2) as i32;
//         // Check for signed division overflow.
//         if ((dividend as u32) != 0x80000000) || divisor != -1 {
//             self.wx(rd) = if divisor != 0 {
//                 (dividend.wrapping_div(divisor)) as u32
//             } else {
//                 u32::MAX // -1.
//             }
//         } else {
//             // Signed division overflow occurred.
//             self.wx(rd, dividend as u32);
//         }
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn divu(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let dividend = self.rx(rs1);
//         let divisor = self.rx(rs2);
//         self.wx(rd) = if divisor != 0 {
//             dividend.wrapping_div(divisor)
//         } else {
//             u32::MAX // -1.
//         };
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn rem(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let dividend = self.rx(rs1) as i32;
//         let divisor = self.rx(rs2) as i32;
//         // Check for signed division overflow.
//         if ((dividend as u32) != 0x80000000) || divisor != -1 {
//             self.wx(rd) = if divisor != 0 {
//                 (dividend % divisor) as u32
//             } else {
//                 dividend as u32
//             }
//         } else {
//             // Signed division overflow occurred.
//             self.wx(rd, 0);
//         }
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }

//     fn remu(&mut self, rd: usize, rs1: usize, rs2: usize) {
//         let dividend = self.rx(rs1);
//         let divisor = self.rx(rs2);
//         self.wx(rd) = if divisor != 0 {
//             dividend % divisor
//         } else {
//             dividend
//         };
//         self.wpc(self.rpc().wrapping_add(4));
//         self.wx(0, 0);
//     }
// }
