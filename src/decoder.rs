use super::{
    cpu_types::{DecodeRv32c, DecodeRv32i, DecodeRv32m},
    tobits::ToBits,
};

pub fn decode<T, U>(decoder: &mut T, code: u32) -> U
where
    T: DecodeRv32i
        + DecodeRv32i<Item = U>
        + DecodeRv32c
        + DecodeRv32c<Item = U>
        + DecodeRv32m
        + DecodeRv32m<Item = U>,
{
    // This function is generated by make_decoder.py. Do not edit.
    let c = ToBits(code);
    match c.c_op() {
        0b00 => match c.c_funct3() {
            0b000 => return decoder.c_addi4spn(c.rdp(), c.c_nzuimm10()),
            0b010 => return decoder.c_lw(c.rdp(), c.rs1p(), c.c_uimm7()),
            0b110 => return decoder.c_sw(c.rs1p(), c.rs2p(), c.c_uimm7()),
            _ => {}
        },
        0b01 => {
            match c.c_funct2() {
                0b00 => {
                    if c.bits(12, 10) == 0b011 {
                        if c.c_funct3() == 0b100 {
                            return decoder.c_sub(c.rdrs1p(), c.rs2p());
                        }
                    }
                }
                0b01 => {
                    if c.bits(12, 10) == 0b011 {
                        if c.c_funct3() == 0b100 {
                            return decoder.c_xor(c.rdrs1p(), c.rs2p());
                        }
                    }
                }
                0b10 => {
                    if c.bits(12, 10) == 0b011 {
                        if c.c_funct3() == 0b100 {
                            return decoder.c_or(c.rdrs1p(), c.rs2p());
                        }
                    }
                }
                0b11 => {
                    if c.bits(12, 10) == 0b011 {
                        if c.c_funct3() == 0b100 {
                            return decoder.c_and(c.rdrs1p(), c.rs2p());
                        }
                    }
                }
                _ => {}
            }
            match c.rd_bits() {
                0b00000 => {
                    if c.c_funct3() == 0b000 {
                        return decoder.c_nop(c.c_nzimm6());
                    }
                }
                0b00010 => {
                    if c.c_funct3() == 0b011 {
                        return decoder.c_addi16sp(c.c_nzimm10());
                    }
                }
                _ => {}
            }
            match c.bits(11, 10) {
                0b00 => {
                    if c.c_funct3() == 0b100 {
                        return decoder.c_srli(c.rdrs1p(), c.c_nzuimm6());
                    }
                }
                0b01 => {
                    if c.c_funct3() == 0b100 {
                        return decoder.c_srai(c.rdrs1p(), c.c_nzuimm6());
                    }
                }
                0b10 => {
                    if c.c_funct3() == 0b100 {
                        return decoder.c_andi(c.rdrs1p(), c.c_imm6());
                    }
                }
                _ => {}
            }
            match c.c_funct3() {
                0b000 => return decoder.c_addi(c.rdrs1n0(), c.c_nzimm6()),
                0b001 => return decoder.c_jal(c.c_imm12()),
                0b010 => return decoder.c_li(c.rd(), c.c_imm6()),
                0b011 => return decoder.c_lui(c.rdn2(), c.c_nzimm18()),
                0b101 => return decoder.c_j(c.c_imm12()),
                0b110 => return decoder.c_beqz(c.rs1p(), c.c_bimm9()),
                0b111 => return decoder.c_bnez(c.rs1p(), c.c_bimm9()),
                _ => {}
            }
        }
        0b10 => {
            if c.bits(6, 2) == 0b00000 {
                match c.bits(12, 12) {
                    0b0 => {
                        if c.c_funct3() == 0b100 {
                            return decoder.c_jr(c.rs1n0());
                        }
                    }
                    0b1 => {
                        if c.c_funct3() == 0b100 {
                            return decoder.c_jalr(c.rs1n0());
                        }
                    }
                    _ => {}
                }
            }
            if c.bits(11, 2) == 0b0000000000 {
                if c.bits(12, 12) == 0b1 {
                    if c.c_funct3() == 0b100 {
                        return decoder.c_ebreak();
                    }
                }
            }
            match c.bits(12, 12) {
                0b0 => {
                    if c.c_funct3() == 0b100 {
                        return decoder.c_mv(c.rd(), c.rs2n0());
                    }
                }
                0b1 => {
                    if c.c_funct3() == 0b100 {
                        return decoder.c_add(c.rdrs1(), c.rs2n0());
                    }
                }
                _ => {}
            }
            match c.c_funct3() {
                0b000 => return decoder.c_slli(c.rdrs1n0(), c.c_nzuimm6()),
                0b010 => return decoder.c_lwsp(c.rdn0(), c.c_uimm8sp()),
                0b110 => return decoder.c_swsp(c.c_rs2(), c.c_uimm8sp_s()),
                _ => {}
            }
        }
        _ => {}
    }
    match c.opcode() {
        0b0000011 => match c.funct3() {
            0b000 => return decoder.lb(c.rd(), c.rs1(), c.iimmediate()),
            0b001 => return decoder.lh(c.rd(), c.rs1(), c.iimmediate()),
            0b010 => return decoder.lw(c.rd(), c.rs1(), c.iimmediate()),
            0b100 => return decoder.lbu(c.rd(), c.rs1(), c.iimmediate()),
            0b101 => return decoder.lhu(c.rd(), c.rs1(), c.iimmediate()),
            _ => {}
        },
        0b0001111 => {
            if c.funct3() == 0b000 {
                return decoder.fence(c.fm(), c.rd(), c.rs1());
            }
        }
        0b0010011 => match c.funct3() {
            0b000 => return decoder.addi(c.rd(), c.rs1(), c.iimmediate()),
            0b001 => {
                if c.funct7() == 0b0000000 {
                    return decoder.slli(c.rd(), c.rs1(), c.shamtw());
                }
            }
            0b010 => return decoder.slti(c.rd(), c.rs1(), c.iimmediate()),
            0b011 => return decoder.sltiu(c.rd(), c.rs1(), c.iimmediate()),
            0b100 => return decoder.xori(c.rd(), c.rs1(), c.iimmediate()),
            0b101 => match c.funct7() {
                0b0000000 => return decoder.srli(c.rd(), c.rs1(), c.shamtw()),
                0b0100000 => return decoder.srai(c.rd(), c.rs1(), c.shamtw()),
                _ => {}
            },
            0b110 => return decoder.ori(c.rd(), c.rs1(), c.iimmediate()),
            0b111 => return decoder.andi(c.rd(), c.rs1(), c.iimmediate()),
            _ => {}
        },
        0b0010111 => return decoder.auipc(c.rd(), c.uimmediate()),
        0b0100011 => match c.funct3() {
            0b000 => return decoder.sb(c.rs1(), c.rs2(), c.simmediate()),
            0b001 => return decoder.sh(c.rs1(), c.rs2(), c.simmediate()),
            0b010 => return decoder.sw(c.rs1(), c.rs2(), c.simmediate()),
            _ => {}
        },
        0b0110011 => match c.funct3() {
            0b000 => match c.funct7() {
                0b0000000 => return decoder.add(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.mul(c.rd(), c.rs1(), c.rs2()),
                0b0100000 => return decoder.sub(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b001 => match c.funct7() {
                0b0000000 => return decoder.sll(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.mulh(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b010 => match c.funct7() {
                0b0000000 => return decoder.slt(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.mulhsu(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b011 => match c.funct7() {
                0b0000000 => return decoder.sltu(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.mulhu(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b100 => match c.funct7() {
                0b0000000 => return decoder.xor(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.div(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b101 => match c.funct7() {
                0b0000000 => return decoder.srl(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.divu(c.rd(), c.rs1(), c.rs2()),
                0b0100000 => return decoder.sra(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b110 => match c.funct7() {
                0b0000000 => return decoder.or(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.rem(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            0b111 => match c.funct7() {
                0b0000000 => return decoder.and(c.rd(), c.rs1(), c.rs2()),
                0b0000001 => return decoder.remu(c.rd(), c.rs1(), c.rs2()),
                _ => {}
            },
            _ => {}
        },
        0b0110111 => return decoder.lui(c.rd(), c.uimmediate()),
        0b1100011 => match c.funct3() {
            0b000 => return decoder.beq(c.rs1(), c.rs2(), c.bimmediate()),
            0b001 => return decoder.bne(c.rs1(), c.rs2(), c.bimmediate()),
            0b100 => return decoder.blt(c.rs1(), c.rs2(), c.bimmediate()),
            0b101 => return decoder.bge(c.rs1(), c.rs2(), c.bimmediate()),
            0b110 => return decoder.bltu(c.rs1(), c.rs2(), c.bimmediate()),
            0b111 => return decoder.bgeu(c.rs1(), c.rs2(), c.bimmediate()),
            _ => {}
        },
        0b1100111 => {
            if c.funct3() == 0b000 {
                return decoder.jalr(c.rd(), c.rs1(), c.iimmediate());
            }
        }
        0b1101111 => return decoder.jal(c.rd(), c.jimmediate()),
        0b1110011 => {
            if c.rd_bits() == 0b00000 {
                if c.funct3() == 0b000 {
                    if c.rs1_bits() == 0b00000 {
                        match c.funct12() {
                            0b000000000000 => return decoder.ecall(),
                            0b000000000001 => return decoder.ebreak(),
                            _ => {}
                        }
                    }
                }
            }
        }
        _ => {}
    }
    decoder.illegal(code)
}
