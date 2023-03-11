

impl<T, U> Rv32i for Cpu<T, U>
where
    T: Mem,
    U: TrapHandler<Item = ()>,
{
    type Item = ();

    // Illegal instruction.

    fn illegal(&mut self, ins: u32) {}

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

    fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // Signed.
        // rd <- (rs1 < rs2) ? 1 : 0, pc += 4
        let xreg_rs1 = self.xreg[rs1] as i32;
        let xreg_rs2 = self.xreg[rs2] as i32;
        self.xreg[rd] = if xreg_rs1 < xreg_rs2 { 1 } else { 0 };
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

    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 ^ rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1] ^ self.xreg[rs2];
        self.pc = self.pc.wrapping_add(4);
        self.xreg[0] = 0;
    }

    fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 >> (rs2 % XLEN), pc += 4
        self.xreg[rd] = self.xreg[rs1] >> (self.xreg[rs2] % 32);
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

    fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 & rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1] & self.xreg[rs2];
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

    // Fence instructions.
    fn fence(&mut self, fm: usize, rd: usize, rs1: usize) {}

    // System instructions.
    fn ecall(&mut self) {}
    fn ebreak(&mut self) {}
}

impl<T, U> Rv32m for Cpu<T, U>
where
    T: Mem,
    U: TrapHandler<Item = ()>,
{
    fn mul(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // rd <- rs1 * rs2, pc += 4
        self.xreg[rd] = self.xreg[rs1].wrapping_mul(self.xreg[rs2]);
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

    fn mulhsu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let xreg_rs1 = (self.xreg[rs1] as i32) as i64;
        let xreg_rs2 = (self.xreg[rs2] as u64) as i64;
        let t = (xreg_rs1 * xreg_rs2) >> 32;
        self.xreg[rd] = t as u32;
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
}

pub trait Rv32f: Rv32i {
    // I-type instructions.
    fn flw(&mut self, rd: usize, rs1: usize, iimm: u32) -> Self::Item;

    // S-type instructions.
    fn fsw(&mut self, rs1: usize, rs2: usize, simm: u32) -> Self::Item;

    // Instructions with rd rs1 rm operands.
    fn fsqrt_s(&mut self, rd: usize, rs1: usize, rm: usize) -> Self::Item;
    fn fcvt_w_s(&mut self, rd: usize, rs1: usize, rm: usize) -> Self::Item;
    fn fcvt_wu_s(&mut self, rd: usize, rs1: usize, rm: usize) -> Self::Item;
    fn fcvt_s_w(&mut self, rd: usize, rs1: usize, rm: usize) -> Self::Item;
    fn fcvt_s_wu(&mut self, rd: usize, rs1: usize, rm: usize) -> Self::Item;

    // Arithmetic instructions.
    fn fadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize) -> Self::Item;
    fn fsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize) -> Self::Item;
    fn fmul_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize) -> Self::Item;
    fn fdiv_s(&mut self, rd: usize, rs1: usize, rs2: usize, rm: usize) -> Self::Item;

    // Fused multiply / add instructions.
    fn fmadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize) -> Self::Item;
    fn fmsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize) -> Self::Item;
    fn fnmsub_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize) -> Self::Item;
    fn fnmadd_s(&mut self, rd: usize, rs1: usize, rs2: usize, rs3: usize, rm: usize) -> Self::Item;

    // Instructions with rd rs1 operands.
    fn fmv_x_w(&mut self, rd: usize, rs1: usize) -> Self::Item;
    fn fmv_w_x(&mut self, rd: usize, rs1: usize) -> Self::Item;
    fn fclass_s(&mut self, rd: usize, rs1: usize) -> Self::Item;

    // Instructions with rd rs1 rs2 operands.
    fn fsgnj_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn fmin_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn fle_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn fsgnjn_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn fmax_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn flt_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn fsgnjx_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
    fn feq_s(&mut self, rd: usize, rs1: usize, rs2: usize) -> Self::Item;
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
