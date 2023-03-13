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
}
