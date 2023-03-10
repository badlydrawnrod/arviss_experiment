pub struct ToBits(pub(crate) u32);

impl ToBits {
    pub fn bits(&self, hi: u32, lo: u32) -> u32 {
        let n = self.0;
        let run = (hi - lo) + 1;
        let mask = ((1 << run) - 1) << lo;
        (n & mask) >> lo
    }

    pub fn opcode(&self) -> u32 {
        self.0 & 0x7f
    }

    pub fn funct3(&self) -> u32 {
        self.bits(14, 12)
    }

    pub fn funct5(&self) -> u32 {
        self.bits(31, 27)
    }

    pub fn funct7(&self) -> u32 {
        self.bits(31, 25)
    }

    pub fn funct12(&self) -> u32 {
        self.bits(31, 20)
    }

    pub fn fmt(&self) -> u32 {
        self.bits(26, 25)
    }

    pub fn fm(&self) -> u32 {
        (self.0 >> 28) & 0xf
    }

    pub fn rd(&self) -> u32 {
        (self.0 >> 7) & 0x1f
    }

    pub fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0x1f
    }

    pub fn rs2(&self) -> u32 {
        (self.0 >> 20) & 0x1f
    }

    pub fn rs3(&self) -> u32 {
        (self.0 >> 27) & 0x1f
    }

    pub fn rm(&self) -> u32 {
        (self.0 >> 12) & 7
    }

    pub fn iimmediate(&self) -> u32 {
        ((self.0 as i32) >> 20) as u32 // inst[31:20] -> sext(imm[11:0])
    }

    pub fn simmediate(&self) -> u32 {
        let imm11_5 = ((self.0 & 0xfe000000) as i32) >> 20; // inst[31:25] -> sext(imm[11:5])
        let imm4_0 = ((self.0 & 0x00000f80) >> 7) as i32; // inst[11:7]  -> imm[4:0]
        (imm11_5 | imm4_0) as u32
    }

    pub fn bimmediate(&self) -> u32 {
        let imm12 = ((self.0 & 0x80000000) as i32) >> 19; // inst[31] -> sext(imm[12])
        let imm11 = ((self.0 & 0x00000080) << 4) as i32; // inst[7] -> imm[11]
        let imm10_5 = ((self.0 & 0x7e000000) >> 20) as i32; // inst[30:25] -> imm[10:5]
        let imm4_1 = ((self.0 & 0x00000f00) >> 7) as i32; // inst[11:8]  -> imm[4:1]
        (imm12 | imm11 | imm10_5 | imm4_1) as u32
    }

    pub fn uimmediate(&self) -> u32 {
        self.0 & 0xfffff000 // inst[31:12] -> imm[31:12]
    }

    pub fn jimmediate(&self) -> u32 {
        let imm20 = ((self.0 & 0x80000000) as i32) >> 11; // inst[31] -> sext(imm[20])
        let imm19_12 = (self.0 & 0x000ff000) as i32; // inst[19:12] -> imm[19:12]
        let imm11 = ((self.0 & 0x00100000) >> 9) as i32; // inst[20] -> imm[11]
        let imm10_1 = ((self.0 & 0x7fe00000) >> 20) as i32; // inst[30:21] -> imm[10:1]
        (imm20 | imm19_12 | imm11 | imm10_1) as u32
    }

    pub fn shamtw(&self) -> u32 {
        (self.0 >> 20) & 0x1f
    }
}
