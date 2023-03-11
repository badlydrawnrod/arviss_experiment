pub struct ToBits(pub u32);

// We know that there are 32 registers. The Rust compiler doesn't, but if we convert to this type then it renders
// bounds checking unnecessary when accesing registers.
#[derive(Clone, Copy, Debug)]
pub enum Reg {
    zero,
    ra,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    s0,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
}

fn to_reg(r: u32) -> Reg {
    match r {
        0 => Reg::zero,
        1 => Reg::ra,
        2 => Reg::sp,
        3 => Reg::gp,
        4 => Reg::tp,
        5 => Reg::t0,
        6 => Reg::t1,
        7 => Reg::t2,
        8 => Reg::s0,
        9 => Reg::s1,
        10 => Reg::a0,
        11 => Reg::a1,
        12 => Reg::a2,
        13 => Reg::a3,
        14 => Reg::a4,
        15 => Reg::a5,
        16 => Reg::a6,
        17 => Reg::a7,
        18 => Reg::s2,
        19 => Reg::s3,
        20 => Reg::s4,
        21 => Reg::s5,
        22 => Reg::s6,
        23 => Reg::s7,
        24 => Reg::s8,
        25 => Reg::s9,
        26 => Reg::s10,
        27 => Reg::s11,
        28 => Reg::t3,
        29 => Reg::t4,
        30 => Reg::t5,
        31 => Reg::t6,
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
