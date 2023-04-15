use crate::{Address, TrapHandler};

use super::{memory::MemoryResult, tobits::Reg, trap_handler::TrapCause};

/// [CoreCpu] represents the basic operations of a CPU that don't involve registers.
///
/// A [CoreCpu] provides an abstraction over the basic CPU operations that don't involve general purpose registers,
/// such as reading and updating the program counter, fetching the next instruction, and reading from and writing to
/// memory.
pub trait CoreCpu {
    /// Returns the current value of the program counter.
    fn get_pc(&self) -> Address;

    /// Sets the program counter to the value last set by `set_next_pc`, retrieves the instruction at the memory address
    /// in the program counter, then sets next pc to the address of the next instruction.
    fn fetch(&mut self) -> MemoryResult<u32>;

    /// Sets the value of next_pc, the address that's copied into the program counter when `fetch` is called.
    fn set_next_pc(&mut self, address: Address);

    /// Reads a byte from memory.
    fn read8(&self, address: Address) -> MemoryResult<u8>;

    /// Reads a 16-bit half-word from memory.
    fn read16(&self, address: Address) -> MemoryResult<u16>;

    /// Reads a 32-bit word from memory.
    fn read32(&self, address: Address) -> MemoryResult<u32>;

    /// Writes a byte to memory.
    fn write8(&mut self, address: Address, value: u8) -> MemoryResult<()>;

    /// Writes a 16-bit half word to memory.
    fn write16(&mut self, address: Address, value: u16) -> MemoryResult<()>;

    /// Writes a 32-bit word to memory.
    fn write32(&mut self, address: Address, value: u32) -> MemoryResult<()>;
}

/// [Xreg] represents access to the base RV32I integer registers.
pub trait Xreg {
    /// Returns the value in the given integer register.
    fn rx(&self, reg: Reg) -> u32;

    /// Writes a value to the given integer register.
    fn wx(&mut self, reg: Reg, val: u32);
}

/// [Freg] represents access to the single-precision floating point registers used by the 'F' extension.
pub trait Freg {
    /// Returns the value in the given floating point register.
    fn rf(&self, reg: Reg) -> f32;

    /// Writes a value to the given floating point register.
    fn wf(&mut self, reg: Reg, val: f32);
}

/// Implements the functions of the base RV32I ISA.
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
    T: CoreCpu + TrapHandler + Xreg,
{
    type Item = ();

    // Illegal instruction.

    fn illegal(&mut self, ins: u32) {
        self.handle_trap(TrapCause::IllegalInstruction(ins));
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
            Err(address) => {
                self.handle_trap(TrapCause::LoadAccessFault(address));
            }
        }
    }

    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- sx(m16(rs1 + imm_i)), pc += 4
        match self.read16(self.rx(rs1).wrapping_add(iimm)) {
            Ok(half_word) => {
                self.wx(rd, ((half_word as i16) as i32) as u32); // TODO: this should be a function.
            }
            Err(address) => {
                self.handle_trap(TrapCause::LoadAccessFault(address));
            }
        }
    }

    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- sx(m32(rs1 + imm_i)), pc += 4
        match self.read32(self.rx(rs1).wrapping_add(iimm)) {
            Ok(word) => {
                self.wx(rd, word);
            }
            Err(address) => {
                self.handle_trap(TrapCause::LoadAccessFault(address));
            }
        }
    }

    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- zx(m8(rs1 + imm_i)), pc += 4
        match self.read8(self.rx(rs1).wrapping_add(iimm)) {
            Ok(byte) => {
                self.wx(rd, byte as u32);
            }
            Err(address) => {
                self.handle_trap(TrapCause::LoadAccessFault(address));
            }
        }
    }

    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) {
        // rd <- zx(m16(rs1 + imm_i)), pc += 4
        match self.read16(self.rx(rs1).wrapping_add(iimm)) {
            Ok(half_word) => {
                self.wx(rd, half_word as u32);
            }
            Err(address) => self.handle_trap(TrapCause::LoadAccessFault(address)),
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
        if let Err(address) =
            self.write8(self.rx(rs1).wrapping_add(simm), (self.rx(rs2) & 0xff) as u8)
        {
            self.handle_trap(TrapCause::StoreAccessFault(address))
        }
    }

    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) {
        // m16(rs1 + imm_s) <- rs2[15:0], pc += 4
        if let Err(address) = self.write16(
            self.rx(rs1).wrapping_add(simm),
            (self.rx(rs2) & 0xffff) as u16,
        ) {
            self.handle_trap(TrapCause::StoreAccessFault(address))
        }
    }

    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) {
        // m32(rs1 + imm_s) <- rs2[31:0], pc += 4
        if let Err(address) = self.write32(self.rx(rs1).wrapping_add(simm), self.rx(rs2)) {
            self.handle_trap(TrapCause::StoreAccessFault(address))
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
    fn ecall(&mut self) {
        self.handle_ecall()
    }

    fn ebreak(&mut self) {
        self.handle_ebreak()
    }
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

pub trait DecodeRv32c {
    type Item;

    fn c_addi4spn(&mut self, rdp: Reg, imm: u32) -> Self::Item;
    fn c_lw(&mut self, rdp: Reg, rs1p: Reg, imm: u32) -> Self::Item;
    fn c_sw(&mut self, rs1p: Reg, rs2p: Reg, imm: u32) -> Self::Item;
    fn c_sub(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item;
    fn c_xor(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item;
    fn c_or(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item;
    fn c_and(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item;
    fn c_nop(&mut self, imm: u32) -> Self::Item;
    fn c_addi16sp(&mut self, imm: u32) -> Self::Item;
    fn c_andi(&mut self, rsrs1p: Reg, imm: u32) -> Self::Item;
    fn c_addi(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item;
    fn c_li(&mut self, rd: Reg, imm: u32) -> Self::Item;
    fn c_lui(&mut self, rdn2: Reg, imm: u32) -> Self::Item;
    fn c_j(&mut self, imm: u32) -> Self::Item;
    fn c_beqz(&mut self, rs1p: Reg, imm: u32) -> Self::Item;
    fn c_bnez(&mut self, rs1p: Reg, imm: u32) -> Self::Item;
    fn c_jr(&mut self, rs1n0: Reg) -> Self::Item;
    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item;
    fn c_ebreak(&mut self) -> Self::Item;
    fn c_mv(&mut self, rd: Reg, rs2n0: Reg) -> Self::Item;
    fn c_add(&mut self, rdrs1: Reg, rs2n0: Reg) -> Self::Item;
    fn c_lwsp(&mut self, rdn0: Reg, imm: u32) -> Self::Item;
    fn c_swsp(&mut self, rs2: Reg, imm: u32) -> Self::Item;
    fn c_jal(&mut self, imm: u32) -> Self::Item;
    fn c_srli(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item;
    fn c_srai(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item;
    fn c_slli(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item;
}

impl<T> DecodeRv32c for T
where
    T: CoreCpu + Xreg + DecodeRv32i,
{
    type Item = ();

    fn c_addi4spn(&mut self, rdp: Reg, imm: u32) -> Self::Item {
        // addi rdp, x2, nzuimm[9:2]
        self.addi(rdp, Reg::SP, imm);
    }

    fn c_lw(&mut self, rdp: Reg, rs1p: Reg, imm: u32) -> Self::Item {
        // lw rdp, offset[6:2](rs1p)
        self.lw(rdp, rs1p, imm);
    }

    fn c_sw(&mut self, rs1p: Reg, rs2p: Reg, imm: u32) -> Self::Item {
        //  sw rs2p, offset[6:2](rs1p)
        self.sw(rs1p, rs2p, imm);
    }

    fn c_sub(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // sub rdp, rdp, rs2p
        self.sub(rdrs1p, rdrs1p, rs2p);
    }

    fn c_xor(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // xor rdp, rdp, rs2p
        self.xor(rdrs1p, rdrs1p, rs2p);
    }

    fn c_or(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // or rdp, rdp, rs2p
        self.or(rdrs1p, rdrs1p, rs2p);
    }

    fn c_and(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        // and rdp, rdp, rs2p
        self.and(rdrs1p, rdrs1p, rs2p);
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        // nop
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        // addi x2, x2, nzimm[9:4]
        self.addi(Reg::SP, Reg::SP, imm);
    }

    fn c_andi(&mut self, rsrs1p: Reg, imm: u32) -> Self::Item {
        // andi rdp, rdp, imm[5:0]
        self.andi(rsrs1p, rsrs1p, imm);
    }

    fn c_addi(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // addi rd, rd, nzimm[5:0]
        self.addi(rdrs1n0, rdrs1n0, imm);
    }

    fn c_li(&mut self, rd: Reg, imm: u32) -> Self::Item {
        // addi rd, x0, imm[5:0]
        self.addi(rd, Reg::ZERO, imm);
    }

    fn c_lui(&mut self, rdn2: Reg, imm: u32) -> Self::Item {
        // lui rd, nzimm[17:12]
        self.lui(rdn2, imm);
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        // jal x0, offset[11:1]
        self.jal(Reg::ZERO, imm);
    }

    fn c_beqz(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // beq rs1p, x0, offset[8:1]
        self.beq(rs1p, Reg::ZERO, imm);
    }

    fn c_bnez(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // bne rs1p, x0, offset[8:1]
        self.bne(rs1p, Reg::ZERO, imm);
    }

    fn c_jr(&mut self, rs1n0: Reg) -> Self::Item {
        // jalr x0, 0(rs1)
        self.jalr(Reg::ZERO, rs1n0, 0);
    }

    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item {
        // jalr x1, 0(rs1)
        self.jalr(Reg::RA, rs1n0, 0);
    }

    fn c_ebreak(&mut self) -> Self::Item {
        self.ebreak();
    }

    fn c_mv(&mut self, rd: Reg, rs2n0: Reg) -> Self::Item {
        // add rd, x0, rs2
        self.add(rd, Reg::ZERO, rs2n0);
    }

    fn c_add(&mut self, rdrs1: Reg, rs2n0: Reg) -> Self::Item {
        // add rd, rd, rs2
        self.add(rdrs1, rdrs1, rs2n0);
    }

    fn c_lwsp(&mut self, rdn0: Reg, imm: u32) -> Self::Item {
        // lw rd, offset[7:2](x2)
        self.lw(rdn0, Reg::SP, imm);
    }

    fn c_swsp(&mut self, rs2: Reg, imm: u32) -> Self::Item {
        // sw rs2, offset[7:2](x2)
        self.sw(Reg::SP, rs2, imm);
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        // jal x1, offset[11:1]
        self.jal(Reg::RA, imm);
    }

    fn c_srli(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // srli rdp, drp, shamt[5:0]
        self.srli(rdrs1p, rdrs1p, imm);
    }

    fn c_srai(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // srai rdp, rdp, shamt[5:0]
        self.srai(rdrs1p, rdrs1p, imm);
    }

    fn c_slli(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // slli rd, rd, shamt[5:0]
        self.slli(rdrs1n0, rdrs1n0, imm);
    }
}

pub trait DecodeRv32f {
    type Item;

    // I-type instructions.
    fn flw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item;

    // S-type instructions.
    fn fsw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item;

    // Instructions with rd rs1 rm operands.
    fn fsqrt_s(&mut self, rd: Reg, rs1: Reg, rm: u32) -> Self::Item;
    fn fcvt_w_s(&mut self, rd: Reg, rs1: Reg, rm: u32) -> Self::Item;
    fn fcvt_wu_s(&mut self, rd: Reg, rs1: Reg, rm: u32) -> Self::Item;
    fn fcvt_s_w(&mut self, rd: Reg, rs1: Reg, rm: u32) -> Self::Item;
    fn fcvt_s_wu(&mut self, rd: Reg, rs1: Reg, rm: u32) -> Self::Item;

    // Arithmetic instructions.
    fn fadd_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rm: u32) -> Self::Item;
    fn fsub_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rm: u32) -> Self::Item;
    fn fmul_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rm: u32) -> Self::Item;
    fn fdiv_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rm: u32) -> Self::Item;

    // Fused multiply / add instructions.
    fn fmadd_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, rm: u32) -> Self::Item;
    fn fmsub_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, rm: u32) -> Self::Item;
    fn fnmsub_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, rm: u32) -> Self::Item;
    fn fnmadd_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, rm: u32) -> Self::Item;

    // Instructions with rd rs1 operands.
    fn fmv_x_w(&mut self, rd: Reg, rs1: Reg) -> Self::Item;
    fn fmv_w_x(&mut self, rd: Reg, rs1: Reg) -> Self::Item;
    fn fclass_s(&mut self, rd: Reg, rs1: Reg) -> Self::Item;

    // Instructions with rd rs1 rs2 operands.
    fn fsgnj_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn fmin_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn fle_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn fsgnjn_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn fmax_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn flt_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn fsgnjx_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
    fn feq_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item;
}

impl<T> DecodeRv32f for T
where
    T: CoreCpu + TrapHandler + Xreg + Freg,
{
    type Item = ();

    // I-type instructions.

    fn flw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        // rd <- f32(rs1 + imm_i)
        match self.read32(self.rx(rs1).wrapping_add(iimm)) {
            Ok(word) => {
                self.wf(rd, f32::from_bits(word));
            }
            Err(address) => {
                self.handle_trap(TrapCause::LoadAccessFault(address));
            }
        }
    }

    // S-type instructions.

    fn fsw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        // f32(rs1 + imm_s) = rs2
        let data = f32::to_bits(self.rf(rs2));
        if let Err(address) = self.write32(self.rx(rs1).wrapping_add(simm), data) {
            self.handle_trap(TrapCause::StoreAccessFault(address));
        }
    }

    // Instructions with rd rs1 rm operands.

    fn fsqrt_s(&mut self, rd: Reg, rs1: Reg, _rm: u32) -> Self::Item {
        // rd <- sqrt(rs1)
        let f = self.rf(rs1);
        self.wf(rd, f32::sqrt(f));
        // TODO: handle rounding modes.
    }

    fn fcvt_w_s(&mut self, rd: Reg, rs1: Reg, _rm: u32) -> Self::Item {
        // rd <- int32_t(rs1)
        let i = self.rf(rs1) as i32;
        self.wx(rd, i as u32);
        // TODO: handle rounding modes.
    }

    fn fcvt_wu_s(&mut self, rd: Reg, rs1: Reg, _rm: u32) -> Self::Item {
        // rd <- uint32_t(rs1)
        let i = self.rf(rs1) as u32;
        self.wx(rd, i);
        // TODO: handle rounding modes.
    }

    fn fcvt_s_w(&mut self, rd: Reg, rs1: Reg, _rm: u32) -> Self::Item {
        // rd <- float(int32_t((rs1))
        let i = self.rx(rs1) as i32;
        self.wf(rd, i as f32);
        // TODO: handle rounding modes.
    }

    fn fcvt_s_wu(&mut self, rd: Reg, rs1: Reg, _rm: u32) -> Self::Item {
        // rd <- float(rs1)
        self.wf(rd, self.rx(rs1) as f32);
        // TODO: handle rounding modes.
    }

    // Arithmetic instructions.

    fn fadd_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, _rm: u32) -> Self::Item {
        // rd <- rs1 + rs2
        self.wf(rd, self.rf(rs1) + self.rf(rs2));
        // TODO: handle rounding modes.
    }

    fn fsub_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, _rm: u32) -> Self::Item {
        // rd <- rs1 - rs2
        self.wf(rd, self.rf(rs1) - self.rf(rs2));
        // TODO: handle rounding modes.
    }

    fn fmul_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, _rm: u32) -> Self::Item {
        // rd <- rs1 * rs2
        self.wf(rd, self.rf(rs1) * self.rf(rs2));
        // TODO: handle rounding modes.
    }

    fn fdiv_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, _rm: u32) -> Self::Item {
        // rd <- rs1 / rs2
        self.wf(rd, self.rf(rs1) / self.rf(rs2));
        // TODO: handle rounding modes.
    }

    // Fused multiply / add instructions.

    fn fmadd_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, _rm: u32) -> Self::Item {
        // rd <- (rs1 * rs2) + rs3
        self.wf(rd, (self.rf(rs1) * self.rf(rs2)) + self.rf(rs3));
        // TODO: handle rounding modes.
    }

    fn fmsub_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, _rm: u32) -> Self::Item {
        // rd <- (rs1 * rs2) - rs3
        self.wf(rd, (self.rf(rs1) * self.rf(rs2)) - self.rf(rs3));
        // TODO: handle rounding modes.
    }

    fn fnmsub_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, _rm: u32) -> Self::Item {
        // rd <- -(rs1 * rs2) + rs3
        self.wf(rd, -(self.rf(rs1) * self.rf(rs2)) + self.rf(rs3));
        // TODO: handle rounding modes.
    }

    fn fnmadd_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg, rs3: Reg, _rm: u32) -> Self::Item {
        // rd <- -(rs1 * rs2) - rs3
        self.wf(rd, -(self.rf(rs1) * self.rf(rs2)) - self.rf(rs3));
        // TODO: handle rounding modes.
    }

    // Instructions with rd rs1 operands.

    fn fmv_x_w(&mut self, rd: Reg, rs1: Reg) -> Self::Item {
        // bits(rd) <- bits(rs1)
        self.wx(rd, f32::to_bits(self.rf(rs1)));
    }

    fn fmv_w_x(&mut self, rd: Reg, rs1: Reg) -> Self::Item {
        // bits(rd) <- bits(rs1)
        self.wf(rd, f32::from_bits(self.rx(rs1)));
    }

    fn fclass_s(&mut self, rd: Reg, rs1: Reg) -> Self::Item {
        let v = self.rf(rs1);
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
        self.wx(rd, result);
    }

    // Instructions with rd rs1 rs2 operands.

    fn fsgnj_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- abs(rs1) * sgn(rs2)
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wf(rd, freg_rs1.abs() * if freg_rs2 < 0.0 { -1.0 } else { 1.0 });
    }

    fn fmin_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- min(rs1, rs2)
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wf(rd, freg_rs1.min(freg_rs2));
    }

    fn fle_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- (rs1 <= rs2) ? 1 : 0;
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wx(rd, if freg_rs1 < freg_rs2 { 1 } else { 0 });
    }

    fn fsgnjn_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- abs(rs1) * -sgn(rs2)
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wf(rd, freg_rs1.abs() * if freg_rs2 < 0.0 { 1.0 } else { -1.0 });
    }

    fn fmax_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- max(rs1, rs2)
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wf(rd, freg_rs1.max(freg_rs2));
    }

    fn flt_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- (rs1 < rs2) ? 1 : 0;
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wx(rd, if freg_rs1 <= freg_rs2 { 1 } else { 0 });
    }

    fn fsgnjx_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- abs(rs1) * (sgn(rs1) == sgn(rs2)) ? 1 : -1
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        // The sign bit is the XOR of the sign bits of rs1 and rs2.
        let m = if (freg_rs1 < 0.0 && freg_rs2 >= 0.0) || (freg_rs1 >= 0.0 && freg_rs2 < 0.0) {
            -1.0
        } else {
            1.0
        };
        self.wf(rd, freg_rs1.abs() * m);
    }

    fn feq_s(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        // rd <- (rs1 == rs2) ? 1 : 0;
        let freg_rs1 = self.rf(rs1);
        let freg_rs2 = self.rf(rs2);
        self.wx(rd, if freg_rs1 == freg_rs2 { 1 } else { 0 });
    }
}
