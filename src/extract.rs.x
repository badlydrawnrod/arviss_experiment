pub fn fm(ins: u32) -> u8 {
    ((ins >> 28) & 0xf) as u8
}

pub fn rd(ins: u32) -> u8 {
    ((ins >> 7) & 0x1f) as u8
}

pub fn rs1(ins: u32) -> u8 {
    ((ins >> 15) & 0x1f) as u8
}

pub fn rs2(ins: u32) -> u8 {
    ((ins >> 20) & 0x1f) as u8
}

pub fn rs3(ins: u32) -> u8 {
    ((ins >> 27) & 0x1f) as u8
}

pub fn rm(ins: u32) -> u8 {
    ((ins >> 12) & 7) as u8
}

pub fn iimmediate(ins: u32) -> i32 {
    (ins as i32) >> 20 // inst[31:20] -> sext(imm[11:0])
}

pub fn simmediate(ins: u32) -> i32 {
    let imm11_5 = ((ins & 0xfe000000) as i32) >> 20; // inst[31:25] -> sext(imm[11:5])
    let imm4_0 = ((ins & 0x00000f80) >> 7) as i32; // inst[11:7]  -> imm[4:0]
    imm11_5 | imm4_0
}

pub fn bimmediate(ins: u32) -> i32 {
    let imm12 = ((ins & 0x80000000) as i32) >> 19; // inst[31] -> sext(imm[12])
    let imm11 = ((ins & 0x00000080) << 4) as i32; // inst[7] -> imm[11]
    let imm10_5 = ((ins & 0x7e000000) >> 20) as i32; // inst[30:25] -> imm[10:5]
    let imm4_1 = ((ins & 0x00000f00) >> 7) as i32; // inst[11:8]  -> imm[4:1]
    imm12 | imm11 | imm10_5 | imm4_1
}

pub fn uimmediate(ins: u32) -> i32 {
    (ins & 0xfffff000) as i32 // inst[31:12] -> imm[31:12]
}

pub fn jimmediate(ins: u32) -> i32 {
    let imm20 = ((ins & 0x80000000) as i32) >> 11; // inst[31] -> sext(imm[20])
    let imm19_12 = (ins & 0x000ff000) as i32; // inst[19:12] -> imm[19:12]
    let imm11 = ((ins & 0x00100000) >> 9) as i32; // inst[20] -> imm[11]
    let imm10_1 = ((ins & 0x7fe00000) >> 20) as i32; // inst[30:21] -> imm[10:1]
    imm20 | imm19_12 | imm11 | imm10_1
}

pub fn shamtw(ins: u32) -> u8 {
    ((ins >> 20) & 0x1f) as u8
}
