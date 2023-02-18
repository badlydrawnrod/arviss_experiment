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


def generate_rust_code(d, level=0):
    spaces = "    "
    indent = spaces * (level + 1)
    i2 = indent + spaces

    if level == 0:
        print(
            "pub fn decode<T>(decoder: &mut (impl Decoder + Decoder<Item = T>), machine_code: u32) -> T {")
        print(f"{indent}// This function is generated by make_decoder.py. Do not edit.")

    for k, v in sorted(d.items()):
        hi, lo = k
        print(f"{indent}match bits(machine_code, {hi}, {lo}) {{")
        for bit_pattern, content in sorted(v.items()):
            print(f"{i2}0x{bit_pattern:x} => ", end="")

            if isinstance(content, dict):
                # We need to decode further, so generate another match recursively.
                print(f"{{")
                generate_rust_code(v[bit_pattern], level + 2)
                print(f"{i2}}}")
            else:
                # We've reached a terminal, so output the function call.

                # Transform the instruction name to an enum, e.g., feq.s becomes FeqS.
                instruction = "".join(s.capitalize()
                                      for s in content[0].split("."))
                opcode = f"{instruction}"

                # Output the call. Function names are based on their operands.
                arg_part = "_".join(s.lower() for s in content[1])
                if len(arg_part) == 0:
                    arg_part = "no_args"
                function_name = f"{arg_part}"
                print(
                    f"return decoder.{function_name}({opcode}, machine_code), // {content[0]}")
        print(f"{i2}_ => {{}}")
        print(f"{indent}}}")

    if level == 0:
        print(f"{indent}// Illegal instruction.")
        print(f"{indent}return decoder.trap(IllegalInstruction, machine_code);")
        print("}")


def generate_rust_code_v2(d, level=0):
    lut = {
        "bimm12hi_bimm12lo_rs1_rs2": "b_type({0}, extract::bimmediate(machine_code), extract::rs1(machine_code), extract::rs2(machine_code))",
        "fm_pred_rd_rs1_succ": "fence({0}, extract::fm(machine_code), extract::rd(machine_code), extract::rs1(machine_code))",
        "imm12_rd_rs1":  "i_type({0}, extract::iimmediate(machine_code), extract::rd(machine_code), extract::rs1(machine_code))",
        "imm12hi_imm12lo_rs1_rs2": "s_type({0}, extract::simmediate(machine_code), extract::rs1(machine_code), extract::rs2(machine_code))",
        "imm20_rd": "u_type({0}, extract::uimmediate(machine_code), extract::rd(machine_code))",
        "jimm20_rd": "j_type({0}, extract::jimmediate(machine_code), extract::rd(machine_code))",
        "no_args": "no_args({0})",
        "rd_rm_rs1": "rd_rm_rs1({0}, extract::rd(machine_code), extract::rm(machine_code), extract::rs1(machine_code))",
        "rd_rm_rs1_rs2": "rd_rm_rs1_rs2({0}, extract::rd(machine_code), extract::rm(machine_code), extract::rs1(machine_code), extract::rs2(machine_code))",
        "rd_rm_rs1_rs2_rs3": "rd_rm_rs1_rs2_rs3({0}, extract::rd(machine_code), extract::rm(machine_code), extract::rs1(machine_code), extract::rs2(machine_code), extract::rs3(machine_code))",
        "rd_rs1": "rd_rs1({0}, extract::rd(machine_code), extract::rs1(machine_code))",
        "rd_rs1_rs2": "rd_rs1_rs2({0}, extract::rd(machine_code), extract::rs1(machine_code), extract::rs2(machine_code))",
        "rd_rs1_shamtw": "rd_rs1_shamtw({0}, extract::rd(machine_code), extract::rs1(machine_code), extract::shamtw(machine_code))",
    }
    spaces = "    "
    indent = spaces * (level + 1)
    i2 = indent + spaces

    if level == 0:
        print(
            "pub fn decode<T>(decoder: &mut (impl Decoder + Decoder<Item = T>), machine_code: u32) -> T {")
        print(f"{indent}// This function is generated by make_decoder.py. Do not edit.")

    for k, v in sorted(d.items()):
        hi, lo = k
        print(f"{indent}match bits(machine_code, {hi}, {lo}) {{")
        for bit_pattern, content in sorted(v.items()):
            print(f"{i2}0x{bit_pattern:x} => ", end="")

            if isinstance(content, dict):
                # We need to decode further, so generate another match recursively.
                print(f"{{")
                generate_rust_code_v2(v[bit_pattern], level + 2)
                print(f"{i2}}}")
            else:
                # We've reached a terminal, so output the function call.

                # Transform the instruction name to an enum, e.g., feq.s becomes FeqS.
                instruction = "".join(s.capitalize()
                                      for s in content[0].split("."))
                opcode = f"{instruction}"

                # Output the call. Function names are based on their operands.
                arg_part = "_".join(s.lower() for s in content[1])
                if len(arg_part) == 0:
                    arg_part = "no_args"
                new_function_name = lut[arg_part].format(opcode)
                print(f"return decoder.{new_function_name}, // {content[0]}")
        print(f"{i2}_ => {{}}")
        print(f"{indent}}}")

    if level == 0:
        print(f"{indent}// Illegal instruction.")
        print(f"{indent}return decoder.trap(IllegalInstruction, machine_code);")
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

beq     bimm12hi rs1 rs2 bimm12lo 14..12=0 6..2=0x18 1..0=3
bne     bimm12hi rs1 rs2 bimm12lo 14..12=1 6..2=0x18 1..0=3
blt     bimm12hi rs1 rs2 bimm12lo 14..12=4 6..2=0x18 1..0=3
bge     bimm12hi rs1 rs2 bimm12lo 14..12=5 6..2=0x18 1..0=3
bltu    bimm12hi rs1 rs2 bimm12lo 14..12=6 6..2=0x18 1..0=3
bgeu    bimm12hi rs1 rs2 bimm12lo 14..12=7 6..2=0x18 1..0=3

jalr    rd rs1 imm12              14..12=0 6..2=0x19 1..0=3

jal     rd jimm20                          6..2=0x1b 1..0=3

lui     rd imm20 6..2=0x0D 1..0=3
auipc   rd imm20 6..2=0x05 1..0=3

addi    rd rs1 imm12           14..12=0 6..2=0x04 1..0=3
slti    rd rs1 imm12           14..12=2 6..2=0x04 1..0=3
sltiu   rd rs1 imm12           14..12=3 6..2=0x04 1..0=3
xori    rd rs1 imm12           14..12=4 6..2=0x04 1..0=3
ori     rd rs1 imm12           14..12=6 6..2=0x04 1..0=3
andi    rd rs1 imm12           14..12=7 6..2=0x04 1..0=3

add     rd rs1 rs2 31..25=0  14..12=0 6..2=0x0C 1..0=3
sub     rd rs1 rs2 31..25=32 14..12=0 6..2=0x0C 1..0=3
sll     rd rs1 rs2 31..25=0  14..12=1 6..2=0x0C 1..0=3
slt     rd rs1 rs2 31..25=0  14..12=2 6..2=0x0C 1..0=3
sltu    rd rs1 rs2 31..25=0  14..12=3 6..2=0x0C 1..0=3
xor     rd rs1 rs2 31..25=0  14..12=4 6..2=0x0C 1..0=3
srl     rd rs1 rs2 31..25=0  14..12=5 6..2=0x0C 1..0=3
sra     rd rs1 rs2 31..25=32 14..12=5 6..2=0x0C 1..0=3
or      rd rs1 rs2 31..25=0  14..12=6 6..2=0x0C 1..0=3
and     rd rs1 rs2 31..25=0  14..12=7 6..2=0x0C 1..0=3

lb      rd rs1       imm12 14..12=0 6..2=0x00 1..0=3
lh      rd rs1       imm12 14..12=1 6..2=0x00 1..0=3
lw      rd rs1       imm12 14..12=2 6..2=0x00 1..0=3
lbu     rd rs1       imm12 14..12=4 6..2=0x00 1..0=3
lhu     rd rs1       imm12 14..12=5 6..2=0x00 1..0=3

sb     imm12hi rs1 rs2 imm12lo 14..12=0 6..2=0x08 1..0=3
sh     imm12hi rs1 rs2 imm12lo 14..12=1 6..2=0x08 1..0=3
sw     imm12hi rs1 rs2 imm12lo 14..12=2 6..2=0x08 1..0=3

fence       fm            pred succ     rs1 14..12=0 rd 6..2=0x03 1..0=3
# fence.i     imm12                       rs1 14..12=1 rd 6..2=0x03 1..0=3

# shifts

slli rd rs1 31..25=0  shamtw  14..12=1 6..2=0x04 1..0=3
srli rd rs1 31..25=0  shamtw  14..12=5 6..2=0x04 1..0=3
srai rd rs1 31..25=32 shamtw  14..12=5 6..2=0x04 1..0=3

# rv32m

mul     rd rs1 rs2 31..25=1 14..12=0 6..2=0x0C 1..0=3
mulh    rd rs1 rs2 31..25=1 14..12=1 6..2=0x0C 1..0=3
mulhsu  rd rs1 rs2 31..25=1 14..12=2 6..2=0x0C 1..0=3
mulhu   rd rs1 rs2 31..25=1 14..12=3 6..2=0x0C 1..0=3
div     rd rs1 rs2 31..25=1 14..12=4 6..2=0x0C 1..0=3
divu    rd rs1 rs2 31..25=1 14..12=5 6..2=0x0C 1..0=3
rem     rd rs1 rs2 31..25=1 14..12=6 6..2=0x0C 1..0=3
remu    rd rs1 rs2 31..25=1 14..12=7 6..2=0x0C 1..0=3

# rv32f

fadd.s    rd rs1 rs2      31..27=0x00 rm       26..25=0 6..2=0x14 1..0=3
fsub.s    rd rs1 rs2      31..27=0x01 rm       26..25=0 6..2=0x14 1..0=3
fmul.s    rd rs1 rs2      31..27=0x02 rm       26..25=0 6..2=0x14 1..0=3
fdiv.s    rd rs1 rs2      31..27=0x03 rm       26..25=0 6..2=0x14 1..0=3
fsgnj.s   rd rs1 rs2      31..27=0x04 14..12=0 26..25=0 6..2=0x14 1..0=3
fsgnjn.s  rd rs1 rs2      31..27=0x04 14..12=1 26..25=0 6..2=0x14 1..0=3
fsgnjx.s  rd rs1 rs2      31..27=0x04 14..12=2 26..25=0 6..2=0x14 1..0=3
fmin.s    rd rs1 rs2      31..27=0x05 14..12=0 26..25=0 6..2=0x14 1..0=3
fmax.s    rd rs1 rs2      31..27=0x05 14..12=1 26..25=0 6..2=0x14 1..0=3
fsqrt.s   rd rs1 24..20=0 31..27=0x0B rm       26..25=0 6..2=0x14 1..0=3

fle.s     rd rs1 rs2      31..27=0x14 14..12=0 26..25=0 6..2=0x14 1..0=3
flt.s     rd rs1 rs2      31..27=0x14 14..12=1 26..25=0 6..2=0x14 1..0=3
feq.s     rd rs1 rs2      31..27=0x14 14..12=2 26..25=0 6..2=0x14 1..0=3

fcvt.w.s  rd rs1 24..20=0 31..27=0x18 rm       26..25=0 6..2=0x14 1..0=3
fcvt.wu.s rd rs1 24..20=1 31..27=0x18 rm       26..25=0 6..2=0x14 1..0=3
fmv.x.w   rd rs1 24..20=0 31..27=0x1C 14..12=0 26..25=0 6..2=0x14 1..0=3
fclass.s  rd rs1 24..20=0 31..27=0x1C 14..12=1 26..25=0 6..2=0x14 1..0=3

fcvt.s.w  rd rs1 24..20=0 31..27=0x1A rm       26..25=0 6..2=0x14 1..0=3
fcvt.s.wu rd rs1 24..20=1 31..27=0x1A rm       26..25=0 6..2=0x14 1..0=3
fmv.w.x   rd rs1 24..20=0 31..27=0x1E 14..12=0 26..25=0 6..2=0x14 1..0=3

flw       rd rs1 imm12 14..12=2 6..2=0x01 1..0=3

fsw       imm12hi rs1 rs2 imm12lo 14..12=2 6..2=0x09 1..0=3

fmadd.s   rd rs1 rs2 rs3 rm 26..25=0 6..2=0x10 1..0=3
fmsub.s   rd rs1 rs2 rs3 rm 26..25=0 6..2=0x11 1..0=3
fnmsub.s  rd rs1 rs2 rs3 rm 26..25=0 6..2=0x12 1..0=3
fnmadd.s  rd rs1 rs2 rs3 rm 26..25=0 6..2=0x13 1..0=3

# system

ecall     11..7=0 19..15=0 31..20=0x000 14..12=0 6..2=0x1C 1..0=3
ebreak    11..7=0 19..15=0 31..20=0x001 14..12=0 6..2=0x1C 1..0=3
# sret      11..7=0 19..15=0 31..20=0x102 14..12=0 6..2=0x1C 1..0=3
# mret      11..7=0 19..15=0 31..20=0x302 14..12=0 6..2=0x1C 1..0=3

# rv554a: I, Allegro: https://music.youtube.com/watch?v=2m0Hp28FS4k&feature=share
"""

if __name__ == "__main__":
    decoded = decode_to_tree(opcodes_to_parse.split("\n"))

    # generate_c_code(decoded)
    # generate_rust_code(decoded)
    generate_rust_code_v2(decoded)
