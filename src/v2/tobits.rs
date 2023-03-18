pub struct ToBits(pub u32);

// We know that there are 32 registers. The Rust compiler doesn't, but if we convert to this type then it renders
// bounds checking unnecessary when accesing registers.
#[derive(Clone, Copy, Debug)]
pub enum Reg {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

fn to_reg(r: u32) -> Reg {
    match r {
        0 => Reg::Zero,
        1 => Reg::Ra,
        2 => Reg::Sp,
        3 => Reg::Gp,
        4 => Reg::Tp,
        5 => Reg::T0,
        6 => Reg::T1,
        7 => Reg::T2,
        8 => Reg::S0,
        9 => Reg::S1,
        10 => Reg::A0,
        11 => Reg::A1,
        12 => Reg::A2,
        13 => Reg::A3,
        14 => Reg::A4,
        15 => Reg::A5,
        16 => Reg::A6,
        17 => Reg::A7,
        18 => Reg::S2,
        19 => Reg::S3,
        20 => Reg::S4,
        21 => Reg::S5,
        22 => Reg::S6,
        23 => Reg::S7,
        24 => Reg::S8,
        25 => Reg::S9,
        26 => Reg::S10,
        27 => Reg::S11,
        28 => Reg::T3,
        29 => Reg::T4,
        30 => Reg::T5,
        31 => Reg::T6,
        _ => unreachable!(),
    }
}

fn creg(r: u32) -> Reg {
    to_reg(8 + r)
}

#[inline]
fn sext(n: u32, top_bit: i32) -> u32 {
    let shift = 31 - top_bit;
    (((n << shift) as i32) >> shift) as u32
}

impl ToBits {
    #[inline]
    pub fn bits(&self, hi: u32, lo: u32) -> u32 {
        let n = self.0;
        let run = (hi - lo) + 1;
        let mask = ((1 << run) - 1) << lo;
        (n & mask) >> lo
    }

    #[inline]
    pub fn opcode(&self) -> u32 {
        self.0 & 0x7f
    }

    #[inline]
    pub fn funct3(&self) -> u32 {
        self.bits(14, 12)
    }

    #[inline]
    pub fn funct5(&self) -> u32 {
        self.bits(31, 27)
    }

    #[inline]
    pub fn funct7(&self) -> u32 {
        self.bits(31, 25)
    }

    #[inline]
    pub fn funct12(&self) -> u32 {
        self.bits(31, 20)
    }

    #[inline]
    pub fn fmt(&self) -> u32 {
        self.bits(26, 25)
    }

    #[inline]
    pub fn fm(&self) -> u32 {
        (self.0 >> 28) & 0xf
    }

    #[inline]
    pub fn rd(&self) -> Reg {
        to_reg((self.0 >> 7) & 0x1f)
    }

    #[inline]
    pub fn rd_bits(&self) -> u32 {
        (self.0 >> 7) & 0x1f
    }

    #[inline]
    pub fn rs1(&self) -> Reg {
        to_reg((self.0 >> 15) & 0x1f)
    }

    #[inline]
    pub fn rs1_bits(&self) -> u32 {
        (self.0 >> 15) & 0x1f
    }

    #[inline]
    pub fn rs2(&self) -> Reg {
        to_reg((self.0 >> 20) & 0x1f)
    }

    #[inline]
    pub fn rs3(&self) -> Reg {
        to_reg((self.0 >> 27) & 0x1f)
    }

    #[inline]
    pub fn rm(&self) -> u32 {
        (self.0 >> 12) & 7
    }

    #[inline]
    pub fn iimmediate(&self) -> u32 {
        ((self.0 as i32) >> 20) as u32 // inst[31:20] -> sext(imm[11:0])
    }

    #[inline]
    pub fn simmediate(&self) -> u32 {
        let imm11_5 = ((self.0 & 0xfe000000) as i32) >> 20; // inst[31:25] -> sext(imm[11:5])
        let imm4_0 = ((self.0 & 0x00000f80) >> 7) as i32; // inst[11:7]  -> imm[4:0]
        (imm11_5 | imm4_0) as u32
    }

    #[inline]
    pub fn bimmediate(&self) -> u32 {
        let imm12 = ((self.0 & 0x80000000) as i32) >> 19; // inst[31] -> sext(imm[12])
        let imm11 = ((self.0 & 0x00000080) << 4) as i32; // inst[7] -> imm[11]
        let imm10_5 = ((self.0 & 0x7e000000) >> 20) as i32; // inst[30:25] -> imm[10:5]
        let imm4_1 = ((self.0 & 0x00000f00) >> 7) as i32; // inst[11:8]  -> imm[4:1]
        (imm12 | imm11 | imm10_5 | imm4_1) as u32
    }

    #[inline]
    pub fn uimmediate(&self) -> u32 {
        self.0 & 0xfffff000 // inst[31:12] -> imm[31:12]
    }

    #[inline]
    pub fn jimmediate(&self) -> u32 {
        let imm20 = ((self.0 & 0x80000000) as i32) >> 11; // inst[31] -> sext(imm[20])
        let imm19_12 = (self.0 & 0x000ff000) as i32; // inst[19:12] -> imm[19:12]
        let imm11 = ((self.0 & 0x00100000) >> 9) as i32; // inst[20] -> imm[11]
        let imm10_1 = ((self.0 & 0x7fe00000) >> 20) as i32; // inst[30:21] -> imm[10:1]
        (imm20 | imm19_12 | imm11 | imm10_1) as u32
    }

    #[inline]
    pub fn shamtw(&self) -> u32 {
        (self.0 >> 20) & 0x1f
    }

    // RV32C

    #[inline]
    pub fn c_op(&self) -> u32 {
        self.0 & 3
    }

    #[inline]
    pub fn c_funct3(&self) -> u32 {
        (self.0 >> 13) & 7
    }

    #[inline]
    pub fn c_funct2(&self) -> u32 {
        (self.0 >> 5) & 3
    }

    #[inline]
    pub fn rdp(&self) -> Reg {
        creg((self.0 >> 2) & 7)
    }

    #[inline]
    pub fn rdn0(&self) -> Reg {
        self.rd()
    }

    #[inline]
    pub fn rdn2(&self) -> Reg {
        self.rd()
    }

    #[inline]
    pub fn rdrs1(&self) -> Reg {
        self.rd()
    }

    #[inline]
    pub fn rs1p(&self) -> Reg {
        creg((self.0 >> 7) & 7)
    }

    #[inline]
    pub fn rs2p(&self) -> Reg {
        creg((self.0 >> 2) & 7)
    }

    #[inline]
    pub fn rdrs1p(&self) -> Reg {
        creg((self.0 >> 7) & 7)
    }

    #[inline]
    pub fn rs1n0(&self) -> Reg {
        to_reg((self.0 >> 7) & 0x1f)
    }

    #[inline]
    pub fn rs2n0(&self) -> Reg {
        to_reg((self.0 >> 2) & 0x1f)
    }

    #[inline]
    pub fn rdrs1n0(&self) -> Reg {
        self.rd()
    }

    #[inline]
    pub fn c_rs2(&self) -> Reg {
        to_reg((self.0 >> 2) & 0x1f)
    }

    #[inline]
    pub fn c_nzuimm10(&self) -> u32 {
        // Zero extended.
        let imm = (self.0 >> 5) & 0xff;
        let a = ((imm & 0b11000000) >> 6) << 4; // 5:4
        let b = ((imm & 0b00111100) >> 2) << 6; // 9:6
        let c = ((imm & 0b00000010) >> 1) << 2; // 2
        let d = ((imm & 0b00000001) >> 0) << 3; // 3
        a | b | c | d
    }

    #[inline]
    pub fn c_uimm7(&self) -> u32 {
        // Zero extended.
        let a = ((self.0 >> 12) & 1) << 5; // offset[5]
        let b = ((self.0 & 0b11100) >> 2) << 3; // offset[4:3]
        let c = ((self.0 & 0b00011) >> 0) << 6; // offset[7:6]
        a | b | c
    }

    #[inline]
    pub fn c_nzimm6(&self) -> u32 {
        // Sign extended.
        let a = ((self.0 >> 12) & 1) << 5; // imm[5]
        let b = (self.0 >> 2) & 0x1f; // imm[4:0]
        sext(a | b, 5)
    }

    #[inline]
    pub fn c_nzimm10(&self) -> u32 {
        // Sign extended.
        let a = ((self.0 >> 12) & 1) << 9; // nzimm[9]
        let imm = ((self.0) >> 2) & 0x1f;
        let b = ((imm & 0b10000) >> 4) << 4; // nzimm[4]
        let c = ((imm & 0b01000) >> 3) << 6; // nzimm[6]
        let d = ((imm & 0b00110) >> 1) << 7; // nzimm[8:7]
        let e = ((imm & 0b00001) >> 0) << 5; // nzimm[5]
        sext(a | b | c | d | e, 9)
    }

    #[inline]
    pub fn c_nzimm18(&self) -> u32 {
        // Sign extended.
        let a = ((self.0 >> 12) & 1) << 17; // nzimm[17]
        let b = ((self.0 >> 2) & 0x1f) << 12; // nzimm[16:12]
        sext(a | b, 17)
    }

    #[inline]
    pub fn c_imm6(&self) -> u32 {
        // Sign extended.
        let a = ((self.0 >> 12) & 1) << 5; // imm[5]
        let b = (self.0 >> 2) & 0x1f; // imm[4:0]
        sext(a | b, 5)
    }

    #[inline]
    pub fn c_imm12(&self) -> u32 {
        // Sign extended.
        let imm = (self.0 >> 2) & 0x7ff;
        let a = ((imm & 0b10000000000) >> 10) << 11; // offset[11]
        let b = ((imm & 0b01000000000) >> 9) << 4; // offset[4]
        let c = ((imm & 0b00110000000) >> 7) << 8; // offset[9:8]
        let d = ((imm & 0b00001000000) >> 6) << 10; // offset[10]
        let e = ((imm & 0b00000100000) >> 5) << 6; // offset[6]
        let f = ((imm & 0b00000010000) >> 4) << 7; // offset[7]
        let g = ((imm & 0b00000001110) >> 1) << 1; // offset[3:1]
        let h = ((imm & 0b00000000001) >> 0) << 5; // offset[5]
        sext(a | b | c | d | e | f | g | h, 11)
    }

    #[inline]
    pub fn c_bimm9(&self) -> u32 {
        // Sign extended.
        let imm1 = (self.0 >> 10) & 7;
        let a = ((imm1 & 0b100) >> 2) << 8; // offset[8]
        let b = ((imm1 & 0b011) >> 0) << 3; // offset[4:3]
        let imm2 = (self.0 >> 2) & 0x1f;
        let c = ((imm2 & 0b11000) >> 3) << 6; // offset[7:6]
        let d = ((imm2 & 0b00110) >> 1) << 1; // offset[2:1]
        let e = ((imm2 & 0b00001) >> 0) << 5; // offset[5]
        sext(a | b | c | d | e, 8)
    }

    #[inline]
    pub fn c_uimm8sp(&self) -> u32 {
        // Zero extended.
        let a = ((self.0 >> 12) & 1) << 5; // offset[5]
        let imm = (self.0 >> 2) & 0x1f;
        let b = ((imm & 0b11100) >> 2) << 2; // offset[4:2]
        let c = ((imm & 0b00011) >> 0) << 6; // offset[7:6]
        a | b | c
    }

    #[inline]
    pub fn c_uimm8sp_s(&self) -> u32 {
        // Zero extended.
        let imm = (self.0 >> 7) & 0x3f;
        let a = ((imm & 0b111100) >> 2) << 2; // offset[5:2]
        let b = ((imm & 0b000011) >> 0) << 6; // offset[7:6]
        a | b
    }

    #[inline]
    pub fn c_nzuimm6(&self) -> u32 {
        // Zero extended.
        let a = ((self.0 >> 12) & 1) << 5; // shamt[5]
        let b = (self.0 >> 2) & 0x1f; // shamt[4:0]
        a | b
    }
}
