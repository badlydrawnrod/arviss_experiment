def generate_c_code(d, level=0):
    if level == 0:
        print("// This function is generated by make_decoder.py. Do not edit.")

    indent = "    " * level
    for k, v in sorted(d.items()):
        hi, lo = k
        print(f"{indent}switch (Bits(ins, {hi}, {lo}))")
        print(f"{indent}{{")
        for bit_pattern, content in sorted(v.items()):
            print(f"{indent}case 0x{bit_pattern:x}:")

            if isinstance(content, dict):
                # We need to decode further, so generate another switch.
                generate_c_code(v[bit_pattern], level + 1)
            else:
                # We've reached a terminal, so output the function call.
                print(f"{indent}    // {content[0]}")

                # Transform the instruction name to an enum, e.g., feq.s becomes execFeqS.
                instruction = "".join(s.capitalize()
                                      for s in content[0].split("."))
                opcode = f"exec{instruction}"

                # Output the call. Function names are based on their operands.
                arg_part = "".join(s.capitalize() for s in content[1])
                if len(arg_part) == 0:
                    arg_part = "NoArgs"
                function_name = f"Gen{arg_part}"
                print(f"{indent}    return {function_name}({opcode}, ins);")
        print(f"{indent}default:")
        print(f"{indent}    break;")
        print(f"{indent}}}")

    if level == 0:
        print(f"// Illegal instruction.")
        print(f"return GenTrap(execIllegalInstruction, ins);")


def generate_rust_code_v3(d, level=0):
    lut = {
        "bimm12hi_bimm12lo_rs1_rs2": "(c.rs1(), c.rs2(), c.bimmediate())",
        "fm_pred_rd_rs1_succ": "(c.fm(), c.rd(), c.rs1())",
        "imm12_rd_rs1": "(c.rd(), c.rs1(), c.iimmediate())",
        "imm12hi_imm12lo_rs1_rs2": "(c.rs1(), c.rs2(), c.simmediate())",
        "imm20_rd": "(c.rd(), c.uimmediate())",
        "jimm20_rd": "(c.rd(), c.jimmediate())",
        "no_args": "()",
        "rd_rm_rs1": "(c.rd(), c.rm(), c.rs1())",
        "rd_rm_rs1_rs2": "(c.rd(), c.rm(), c.rs1(), c.rs2())",
        "rd_rm_rs1_rs2_rs3": "(c.rd(), c.rm(), c.rs1(), c.rs2(), c.rs3())",
        "rd_rs1": "(c.rd(), c.rs1())",
        "rd_rs1_rs2": "(c.rd(), c.rs1(), c.rs2())",
        "rd_rs1_shamtw": "(c.rd(), c.rs1(), c.shamtw())",
    }
    names = {
        (31, 27): "funct5()",
        (31, 25): "funct7()",
        (31, 20): "funct12()",
        (26, 25): "fmt()",
        (24, 20): "rs2()",
        (19, 15): "rs1_bits()",
        (14, 12): "funct3()",
        (11, 7): "rd_bits()",
        (6, 0): "opcode()",
    }
    spaces = "    "
    indent = spaces * (level + 1)
    i2 = indent + spaces

    if level == 0:
        print("""\
pub fn decode<T, U>(decoder: &mut T, code: u32) -> U
where
    T: DecodeRv32i + DecodeRv32i<Item = U>
{""")
        print(f"{indent}// This function is generated by make_decoder.py. Do not edit.")
        print(f"{indent}let c = ToBits(code);")

    for k, v in sorted(d.items()):
        hi, lo = k
        name = names.get((hi, lo), f"bits({hi}, {lo})")
        print(f"{indent}match c.{name} {{")
        for bit_pattern, content in sorted(v.items()):
            width = 1 + (hi - lo)
            print(f"{i2}0b{bit_pattern:0{width}b} => ", end="")

            if isinstance(content, dict):
                # We need to decode further, so generate another match recursively.
                print(f"{{")
                generate_rust_code_v3(v[bit_pattern], level + 2)
                print(f"{i2}}}")
            else:
                # We've reached a terminal, so output the function call.

                # Transform the instruction name to something that Rust can handle, e.g., feq.s becomes feq_s.
                instruction = "_".join(s.lower()
                                       for s in content[0].split("."))
                opcode = f"{instruction}"

                # Output the call. Function names are based on their operands.
                arg_part = "_".join(s.lower() for s in content[1])
                if len(arg_part) == 0:
                    arg_part = "no_args"
                args = lut[arg_part]
                print(f"return decoder.{opcode}{args},")
        print(f"{i2}_ => {{}}")
        print(f"{indent}}}")

    if level == 0:
        print(f"{indent}decoder.illegal(code)")
        print("}")

# From: https://stackoverflow.com/questions/7204805/how-to-merge-dictionaries-of-dictionaries


def merge(a, b, path=None):
    if path is None:
        path = []
    for key in b:
        if key in a:
            if isinstance(a[key], dict) and isinstance(b[key], dict):
                merge(a[key], b[key], path + [str(key)])
            elif a[key] == b[key]:
                pass  # same leaf value
            else:
                raise Exception('Conflict at %s' % '.'.join(path + [str(key)]))
        else:
            a[key] = b[key]
    return a


def decode_to_tree(lines):
    operands = {"rd", "rs1", "rs2", "rs3", "bimm12hi", "bimm12lo", "imm12hi", "imm12lo", "imm12", "jimm20", "imm20",
                "fm", "pred", "succ", "rm", "shamtw", "shamt"}

    result = {}
    for line in lines:
        line = line.strip()
        if len(line) == 0 or line[0] == "#":
            continue
        ins, *rest = line.split()
        t_ops = []
        t_operands = []
        for item in rest:
            if item in operands:
                # It's an operand.
                t_operands.append(item)
            else:
                bit_range, bit_pattern = item.split("=")
                bit_pattern = int(bit_pattern, 0)
                if ".." in bit_range:
                    hi, lo = bit_range.split("..")
                else:
                    hi, lo = bit_range, bit_range
                hi, lo = int(hi), int(lo)
                t_ops.append((hi, lo, bit_pattern))

        # Build a tree from the instruction to the root, then merge it into the main tree.
        node = (ins, sorted(t_operands))
        for hi, lo, bit_pattern in t_ops:
            node = {(hi, lo): {bit_pattern: node}}
        merge(result, node)

    return result


# See: https://github.com/riscv/riscv-opcodes
opcodes_to_parse = """\
# format of a line in this file:
# <instruction name> <args> <opcode>
#
# <opcode> is given by specifying one or more range/value pairs:
# hi..lo=value or bit=value or arg=value (e.g. 6..2=0x45 10=1 rd=0)
#
# <args> is one of rd, rs1, rs2, rs3, imm20, imm12, imm12lo, imm12hi,
# shamtw, shamt, rm

# rv32i

#beq     bimm12hi rs1 rs2 bimm12lo 14..12=0 6..2=0x18 1..0=3
#bne     bimm12hi rs1 rs2 bimm12lo 14..12=1 6..2=0x18 1..0=3
#blt     bimm12hi rs1 rs2 bimm12lo 14..12=4 6..2=0x18 1..0=3
#bge     bimm12hi rs1 rs2 bimm12lo 14..12=5 6..2=0x18 1..0=3
#bltu    bimm12hi rs1 rs2 bimm12lo 14..12=6 6..2=0x18 1..0=3
#bgeu    bimm12hi rs1 rs2 bimm12lo 14..12=7 6..2=0x18 1..0=3
beq     bimm12hi rs1 rs2 bimm12lo 14..12=0 6..0=0b1100011
bne     bimm12hi rs1 rs2 bimm12lo 14..12=1 6..0=0b1100011
blt     bimm12hi rs1 rs2 bimm12lo 14..12=4 6..0=0b1100011
bge     bimm12hi rs1 rs2 bimm12lo 14..12=5 6..0=0b1100011
bltu    bimm12hi rs1 rs2 bimm12lo 14..12=6 6..0=0b1100011
bgeu    bimm12hi rs1 rs2 bimm12lo 14..12=7 6..0=0b1100011

#jalr    rd rs1 imm12              14..12=0 6..2=0x19 1..0=3
jalr    rd rs1 imm12              14..12=0 6..0=0b1100111

#jal     rd jimm20                          6..2=0x1b 1..0=3
jal     rd jimm20                          6..0=0b1101111

# lui     rd imm20 6..2=0x0D 1..0=3
# auipc   rd imm20 6..2=0x05 1..0=3
lui     rd imm20 6..0=0b0110111
auipc   rd imm20 6..0=0b0010111

# addi    rd rs1 imm12           14..12=0 6..2=0x04 1..0=3
# slti    rd rs1 imm12           14..12=2 6..2=0x04 1..0=3
# sltiu   rd rs1 imm12           14..12=3 6..2=0x04 1..0=3
# xori    rd rs1 imm12           14..12=4 6..2=0x04 1..0=3
# ori     rd rs1 imm12           14..12=6 6..2=0x04 1..0=3
# andi    rd rs1 imm12           14..12=7 6..2=0x04 1..0=3
addi    rd rs1 imm12           14..12=0 6..0=0b0010011
slti    rd rs1 imm12           14..12=2 6..0=0b0010011
sltiu   rd rs1 imm12           14..12=3 6..0=0b0010011
xori    rd rs1 imm12           14..12=4 6..0=0b0010011
ori     rd rs1 imm12           14..12=6 6..0=0b0010011
andi    rd rs1 imm12           14..12=7 6..0=0b0010011

# add     rd rs1 rs2 31..25=0  14..12=0 6..2=0x0C 1..0=3
# sub     rd rs1 rs2 31..25=32 14..12=0 6..2=0x0C 1..0=3
# sll     rd rs1 rs2 31..25=0  14..12=1 6..2=0x0C 1..0=3
# slt     rd rs1 rs2 31..25=0  14..12=2 6..2=0x0C 1..0=3
# sltu    rd rs1 rs2 31..25=0  14..12=3 6..2=0x0C 1..0=3
# xor     rd rs1 rs2 31..25=0  14..12=4 6..2=0x0C 1..0=3
# srl     rd rs1 rs2 31..25=0  14..12=5 6..2=0x0C 1..0=3
# sra     rd rs1 rs2 31..25=32 14..12=5 6..2=0x0C 1..0=3
# or      rd rs1 rs2 31..25=0  14..12=6 6..2=0x0C 1..0=3
# and     rd rs1 rs2 31..25=0  14..12=7 6..2=0x0C 1..0=3
add     rd rs1 rs2 31..25=0  14..12=0 6..0=0b0110011
sub     rd rs1 rs2 31..25=32 14..12=0 6..0=0b0110011
sll     rd rs1 rs2 31..25=0  14..12=1 6..0=0b0110011
slt     rd rs1 rs2 31..25=0  14..12=2 6..0=0b0110011
sltu    rd rs1 rs2 31..25=0  14..12=3 6..0=0b0110011
xor     rd rs1 rs2 31..25=0  14..12=4 6..0=0b0110011
srl     rd rs1 rs2 31..25=0  14..12=5 6..0=0b0110011
sra     rd rs1 rs2 31..25=32 14..12=5 6..0=0b0110011
or      rd rs1 rs2 31..25=0  14..12=6 6..0=0b0110011
and     rd rs1 rs2 31..25=0  14..12=7 6..0=0b0110011

# lb      rd rs1       imm12 14..12=0 6..2=0x00 1..0=3
# lh      rd rs1       imm12 14..12=1 6..2=0x00 1..0=3
# lw      rd rs1       imm12 14..12=2 6..2=0x00 1..0=3
# lbu     rd rs1       imm12 14..12=4 6..2=0x00 1..0=3
# lhu     rd rs1       imm12 14..12=5 6..2=0x00 1..0=3
lb      rd rs1       imm12 14..12=0 6..0=0b0000011
lh      rd rs1       imm12 14..12=1 6..0=0b0000011
lw      rd rs1       imm12 14..12=2 6..0=0b0000011
lbu     rd rs1       imm12 14..12=4 6..0=0b0000011
lhu     rd rs1       imm12 14..12=5 6..0=0b0000011

# sb     imm12hi rs1 rs2 imm12lo 14..12=0 6..2=0x08 1..0=3
# sh     imm12hi rs1 rs2 imm12lo 14..12=1 6..2=0x08 1..0=3
# sw     imm12hi rs1 rs2 imm12lo 14..12=2 6..2=0x08 1..0=3
sb     imm12hi rs1 rs2 imm12lo 14..12=0 6..0=0b0100011
sh     imm12hi rs1 rs2 imm12lo 14..12=1 6..0=0b0100011
sw     imm12hi rs1 rs2 imm12lo 14..12=2 6..0=0b0100011

#fence       fm            pred succ     rs1 14..12=0 rd 6..2=0x03 1..0=3
fence       fm            pred succ     rs1 14..12=0 rd 6..0=0b0001111

# ecall     11..7=0 19..15=0 31..20=0x000 14..12=0 6..2=0x1C 1..0=3
# ebreak    11..7=0 19..15=0 31..20=0x001 14..12=0 6..2=0x1C 1..0=3
ecall     11..7=0 19..15=0 31..20=0x000 14..12=0 6..0=0b1110011
ebreak    11..7=0 19..15=0 31..20=0x001 14..12=0 6..0=0b1110011

# fence.i     imm12                       rs1 14..12=1 rd 6..2=0x03 1..0=3

# shifts

# slli rd rs1 31..25=0  shamtw  14..12=1 6..2=0x04 1..0=3
# srli rd rs1 31..25=0  shamtw  14..12=5 6..2=0x04 1..0=3
# srai rd rs1 31..25=32 shamtw  14..12=5 6..2=0x04 1..0=3
slli rd rs1 31..25=0  shamtw  14..12=1 6..0=0b0010011
srli rd rs1 31..25=0  shamtw  14..12=5 6..0=0b0010011
srai rd rs1 31..25=32 shamtw  14..12=5 6..0=0b0010011

# # rv32m

# # mul     rd rs1 rs2 31..25=1 14..12=0 6..2=0x0C 1..0=3
# # mulh    rd rs1 rs2 31..25=1 14..12=1 6..2=0x0C 1..0=3
# # mulhsu  rd rs1 rs2 31..25=1 14..12=2 6..2=0x0C 1..0=3
# # mulhu   rd rs1 rs2 31..25=1 14..12=3 6..2=0x0C 1..0=3
# # div     rd rs1 rs2 31..25=1 14..12=4 6..2=0x0C 1..0=3
# # divu    rd rs1 rs2 31..25=1 14..12=5 6..2=0x0C 1..0=3
# # rem     rd rs1 rs2 31..25=1 14..12=6 6..2=0x0C 1..0=3
# # remu    rd rs1 rs2 31..25=1 14..12=7 6..2=0x0C 1..0=3
# mul     rd rs1 rs2 31..25=1 14..12=0 6..0=0b0110011
# mulh    rd rs1 rs2 31..25=1 14..12=1 6..0=0b0110011
# mulhsu  rd rs1 rs2 31..25=1 14..12=2 6..0=0b0110011
# mulhu   rd rs1 rs2 31..25=1 14..12=3 6..0=0b0110011
# div     rd rs1 rs2 31..25=1 14..12=4 6..0=0b0110011
# divu    rd rs1 rs2 31..25=1 14..12=5 6..0=0b0110011
# rem     rd rs1 rs2 31..25=1 14..12=6 6..0=0b0110011
# remu    rd rs1 rs2 31..25=1 14..12=7 6..0=0b0110011

# # rv32f

# # fadd.s    rd rs1 rs2      31..27=0x00 rm       26..25=0 6..2=0x14 1..0=3
# # fsub.s    rd rs1 rs2      31..27=0x01 rm       26..25=0 6..2=0x14 1..0=3
# # fmul.s    rd rs1 rs2      31..27=0x02 rm       26..25=0 6..2=0x14 1..0=3
# # fdiv.s    rd rs1 rs2      31..27=0x03 rm       26..25=0 6..2=0x14 1..0=3
# # fsgnj.s   rd rs1 rs2      31..27=0x04 14..12=0 26..25=0 6..2=0x14 1..0=3
# # fsgnjn.s  rd rs1 rs2      31..27=0x04 14..12=1 26..25=0 6..2=0x14 1..0=3
# # fsgnjx.s  rd rs1 rs2      31..27=0x04 14..12=2 26..25=0 6..2=0x14 1..0=3
# # fmin.s    rd rs1 rs2      31..27=0x05 14..12=0 26..25=0 6..2=0x14 1..0=3
# # fmax.s    rd rs1 rs2      31..27=0x05 14..12=1 26..25=0 6..2=0x14 1..0=3
# # fsqrt.s   rd rs1 24..20=0 31..27=0x0B rm       26..25=0 6..2=0x14 1..0=3
# fadd.s    rd rs1 rs2      31..27=0x00 rm       26..25=0 6..0=0b1010011
# fsub.s    rd rs1 rs2      31..27=0x01 rm       26..25=0 6..0=0b1010011
# fmul.s    rd rs1 rs2      31..27=0x02 rm       26..25=0 6..0=0b1010011
# fdiv.s    rd rs1 rs2      31..27=0x03 rm       26..25=0 6..0=0b1010011
# fsgnj.s   rd rs1 rs2      31..27=0x04 14..12=0 26..25=0 6..0=0b1010011
# fsgnjn.s  rd rs1 rs2      31..27=0x04 14..12=1 26..25=0 6..0=0b1010011
# fsgnjx.s  rd rs1 rs2      31..27=0x04 14..12=2 26..25=0 6..0=0b1010011
# fmin.s    rd rs1 rs2      31..27=0x05 14..12=0 26..25=0 6..0=0b1010011
# fmax.s    rd rs1 rs2      31..27=0x05 14..12=1 26..25=0 6..0=0b1010011
# fsqrt.s   rd rs1 24..20=0 31..27=0x0B rm       26..25=0 6..0=0b1010011

# # fle.s     rd rs1 rs2      31..27=0x14 14..12=0 26..25=0 6..2=0x14 1..0=3
# # flt.s     rd rs1 rs2      31..27=0x14 14..12=1 26..25=0 6..2=0x14 1..0=3
# # feq.s     rd rs1 rs2      31..27=0x14 14..12=2 26..25=0 6..2=0x14 1..0=3
# fle.s     rd rs1 rs2      31..27=0x14 14..12=0 26..25=0 6..0=0b1010011
# flt.s     rd rs1 rs2      31..27=0x14 14..12=1 26..25=0 6..0=0b1010011
# feq.s     rd rs1 rs2      31..27=0x14 14..12=2 26..25=0 6..0=0b1010011

# # fcvt.w.s  rd rs1 24..20=0 31..27=0x18 rm       26..25=0 6..2=0x14 1..0=3
# # fcvt.wu.s rd rs1 24..20=1 31..27=0x18 rm       26..25=0 6..2=0x14 1..0=3
# # fmv.x.w   rd rs1 24..20=0 31..27=0x1C 14..12=0 26..25=0 6..2=0x14 1..0=3
# # fclass.s  rd rs1 24..20=0 31..27=0x1C 14..12=1 26..25=0 6..2=0x14 1..0=3
# fcvt.w.s  rd rs1 24..20=0 31..27=0x18 rm       26..25=0 6..0=0b1010011
# fcvt.wu.s rd rs1 24..20=1 31..27=0x18 rm       26..25=0 6..0=0b1010011
# fmv.x.w   rd rs1 24..20=0 31..27=0x1C 14..12=0 26..25=0 6..0=0b1010011
# fclass.s  rd rs1 24..20=0 31..27=0x1C 14..12=1 26..25=0 6..0=0b1010011

# # fcvt.s.w  rd rs1 24..20=0 31..27=0x1A rm       26..25=0 6..2=0x14 1..0=3
# # fcvt.s.wu rd rs1 24..20=1 31..27=0x1A rm       26..25=0 6..2=0x14 1..0=3
# # fmv.w.x   rd rs1 24..20=0 31..27=0x1E 14..12=0 26..25=0 6..2=0x14 1..0=3
# fcvt.s.w  rd rs1 24..20=0 31..27=0x1A rm       26..25=0 6..0=0b1010011
# fcvt.s.wu rd rs1 24..20=1 31..27=0x1A rm       26..25=0 6..0=0b1010011
# fmv.w.x   rd rs1 24..20=0 31..27=0x1E 14..12=0 26..25=0 6..0=0b1010011

# #flw       rd rs1 imm12 14..12=2 6..2=0x01 1..0=3
# flw       rd rs1 imm12 14..12=2 6..0=0b0000111

# #fsw       imm12hi rs1 rs2 imm12lo 14..12=2 6..2=0x09 1..0=3
# fsw       imm12hi rs1 rs2 imm12lo 14..12=2 6..0=0b0100111

# # fmadd.s   rd rs1 rs2 rs3 rm 26..25=0 6..2=0x10 1..0=3
# # fmsub.s   rd rs1 rs2 rs3 rm 26..25=0 6..2=0x11 1..0=3
# # fnmsub.s  rd rs1 rs2 rs3 rm 26..25=0 6..2=0x12 1..0=3
# # fnmadd.s  rd rs1 rs2 rs3 rm 26..25=0 6..2=0x13 1..0=3
# fmadd.s   rd rs1 rs2 rs3 rm 26..25=0 6..0=0b1000011
# fmsub.s   rd rs1 rs2 rs3 rm 26..25=0 6..0=0b1000111
# fnmsub.s  rd rs1 rs2 rs3 rm 26..25=0 6..0=0b1001011
# fnmadd.s  rd rs1 rs2 rs3 rm 26..25=0 6..0=0b1001111

# # system

# sret      11..7=0 19..15=0 31..20=0x102 14..12=0 6..2=0x1C 1..0=3
# mret      11..7=0 19..15=0 31..20=0x302 14..12=0 6..2=0x1C 1..0=3

# rv554a: I, Allegro: https://music.youtube.com/watch?v=2m0Hp28FS4k&feature=share
"""

if __name__ == "__main__":
    decoded = decode_to_tree(opcodes_to_parse.split("\n"))
    generate_rust_code_v3(decoded)
