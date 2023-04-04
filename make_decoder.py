import argparse

from dataclasses import dataclass
from typing import List, Tuple

# Format of lines in opcodes.
# <instruction name> <args> <opcode>
#
# <opcode> is given by specifying one or more range/value pairs:
# hi..lo=value or bit=value or arg=value (e.g. 6..2=0x45 10=1 rd=0)
#
# <args> is one of rd, rs1, rs2, rs3, imm20, imm12, imm12lo, imm12hi,
# shamtw, shamt, rm
# See: https://github.com/riscv/riscv-opcodes

rv32i = """\
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

ecall     11..7=0 19..15=0 31..20=0x000 14..12=0 6..2=0x1C 1..0=3
ebreak    11..7=0 19..15=0 31..20=0x001 14..12=0 6..2=0x1C 1..0=3

# fence.i     imm12                       rs1 14..12=1 rd 6..2=0x03 1..0=3

slli rd rs1 31..25=0  shamtw  14..12=1 6..2=0x04 1..0=3
srli rd rs1 31..25=0  shamtw  14..12=5 6..2=0x04 1..0=3
srai rd rs1 31..25=32 shamtw  14..12=5 6..2=0x04 1..0=3
"""

rv32m = """\
mul     rd rs1 rs2 31..25=1 14..12=0 6..2=0x0C 1..0=3
mulh    rd rs1 rs2 31..25=1 14..12=1 6..2=0x0C 1..0=3
mulhsu  rd rs1 rs2 31..25=1 14..12=2 6..2=0x0C 1..0=3
mulhu   rd rs1 rs2 31..25=1 14..12=3 6..2=0x0C 1..0=3
div     rd rs1 rs2 31..25=1 14..12=4 6..2=0x0C 1..0=3
divu    rd rs1 rs2 31..25=1 14..12=5 6..2=0x0C 1..0=3
rem     rd rs1 rs2 31..25=1 14..12=6 6..2=0x0C 1..0=3
remu    rd rs1 rs2 31..25=1 14..12=7 6..2=0x0C 1..0=3
"""

rv32c = """\
# quadrant 0
c.addi4spn rd_p c_nzuimm10              1..0=0 15..13=0
c.lw rd_p rs1_p c_uimm7lo c_uimm7hi     1..0=0 15..13=2
c.sw rs1_p rs2_p c_uimm7lo c_uimm7hi    1..0=0 15..13=6

#quadrant 1
c.nop c_nzimm6hi c_nzimm6lo              1..0=1 15..13=0 11..7=0
c.addi rd_rs1_n0 c_nzimm6lo c_nzimm6hi   1..0=1 15..13=0
c.li rd c_imm6lo c_imm6hi                1..0=1 15..13=2
c.addi16sp c_nzimm10hi c_nzimm10lo       1..0=1 15..13=3 11..7=2
c.lui rd_n2 c_nzimm18hi c_nzimm18lo      1..0=1 15..13=3
c.andi rd_rs1_p c_imm6hi c_imm6lo        1..0=1 15..13=4 11..10=2
c.sub rd_rs1_p rs2_p                     1..0=1 15..13=4 12..10=0b011 6..5=0
c.xor rd_rs1_p rs2_p                     1..0=1 15..13=4 12..10=0b011 6..5=1
c.or rd_rs1_p rs2_p                      1..0=1 15..13=4 12..10=0b011 6..5=2
c.and rd_rs1_p rs2_p                     1..0=1 15..13=4 12..10=0b011 6..5=3
c.j c_imm12                              1..0=1 15..13=5
c.beqz rs1_p c_bimm9lo c_bimm9hi         1..0=1 15..13=6
c.bnez rs1_p c_bimm9lo c_bimm9hi         1..0=1 15..13=7

#quadrant 2
c.lwsp rd_n0 c_uimm8sphi c_uimm8splo     1..0=2 15..13=2
c.jr rs1_n0                              1..0=2 15..13=4 12=0 6..2=0
c.mv rd c_rs2_n0                         1..0=2 15..13=4 12=0              
c.ebreak                                 1..0=2 15..13=4 12=1 11..2=0
c.jalr c_rs1_n0                          1..0=2 15..13=4 12=1 6..2=0
c.add rd_rs1 c_rs2_n0                    1..0=2 15..13=4 12=1 
c.swsp c_rs2 c_uimm8sp_s                 1..0=2 15..13=6

# rv32c only

c.jal c_imm12              1..0=1 15..13=1
c.srli rd_rs1_p c_nzuimm6lo c_nzuimm6hi   1..0=1 15..13=4 11..10=0
c.srai rd_rs1_p c_nzuimm6lo c_nzuimm6hi   1..0=1 15..13=4 11..10=1
c.slli rd_rs1_n0 c_nzuimm6hi c_nzuimm6lo  1..0=2 15..13=0
"""

rv32f = """\
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
"""

system = """\
sret      11..7=0 19..15=0 31..20=0x102 14..12=0 6..2=0x1C 1..0=3
mret      11..7=0 19..15=0 31..20=0x302 14..12=0 6..2=0x1C 1..0=3
"""

# A lovely piece of music by Vivaldi. Don't try to decode it. Just listen and enjoy.
rv554a = """\
rv554a: I, Allegro: https://music.youtube.com/watch?v=2m0Hp28FS4k&feature=share
"""

lut = {
    "bimm12hi rs1 rs2 bimm12lo": "(c.rs1(), c.rs2(), c.bimmediate())",
    "fm pred succ rs1 rd": "(c.fm(), c.rd(), c.rs1())",
    "rd rs1 imm12": "(c.rd(), c.rs1(), c.iimmediate())",
    "imm12hi rs1 rs2 imm12lo": "(c.rs1(), c.rs2(), c.simmediate())",
    "rd imm20": "(c.rd(), c.uimmediate())",
    "rd jimm20": "(c.rd(), c.jimmediate())",
    "no_args": "()",
    "rd_rm_rs1": "(c.rd(), c.rm(), c.rs1())",
    "rd_rm_rs1_rs2": "(c.rd(), c.rm(), c.rs1(), c.rs2())",
    "rd_rm_rs1_rs2_rs3": "(c.rd(), c.rm(), c.rs1(), c.rs2(), c.rs3())",
    "rd rs1": "(c.rd(), c.rs1())",
    "rd rs1 rs2": "(c.rd(), c.rs1(), c.rs2())",
    "rd rs1 shamtw": "(c.rd(), c.rs1(), c.shamtw())",
    # C-extension.
    "rd_p c_nzuimm10": "(c.rdp(), c.c_nzuimm10())",  # c.addi4spn
    "rd_p rs1_p c_uimm7lo c_uimm7hi": "(c.rdp(), c.rs1p(), c.c_uimm7())",  # c.lw
    "rs1_p rs2_p c_uimm7lo c_uimm7hi": "(c.rs1p(), c.rs2p(), c.c_uimm7())",  # c.sw
    "c_nzimm6hi c_nzimm6lo": "(c.c_nzimm6())",  # c.nop
    "rd_rs1_n0 c_nzimm6lo c_nzimm6hi": "(c.rdrs1n0(), c.c_nzimm6())",  # c.addi
    "rd c_imm6lo c_imm6hi": "(c.rd(), c.c_imm6())",  # c.li
    "c_nzimm10hi c_nzimm10lo": "(c.c_nzimm10())",  # c.addi16sp
    "rd_n2 c_nzimm18hi c_nzimm18lo": "(c.rdn2(), c.c_nzimm18())",  # c.lui
    "rd_rs1_p c_imm6hi c_imm6lo": "(c.rdrs1p(), c.c_imm6())",  # c.andi
    "rd_rs1_p rs2_p": "(c.rdrs1p(), c.rs2p())",  # c.sub, c.xor, c.or, c.and
    "c_imm12": "(c.c_imm12())",  # c.j, c.jal
    "rs1_p c_bimm9lo c_bimm9hi": "(c.rs1p(), c.c_bimm9())",  # c.beqz, c.bnez
    "rd_n0 c_uimm8sphi c_uimm8splo": "(c.rdn0(), c.c_uimm8sp())",  # c.lwsp
    "rs1_n0": "(c.rs1n0())",  # c.jr
    "rd c_rs2_n0": "(c.rd(), c.rs2n0())",  # c.mv
    "c_rs1_n0": "(c.rs1n0())",  # c.jalr
    "rd_rs1 c_rs2_n0": "(c.rdrs1(), c.rs2n0())",  # c.add
    "c_rs2 c_uimm8sp_s": "(c.c_rs2(), c.c_uimm8sp_s())",  # c.swsp
    "rd_rs1_p c_nzuimm6lo c_nzuimm6hi": "(c.rdrs1p(), c.c_nzuimm6())",  # c.srli, c.srai
    "rd_rs1_n0 c_nzuimm6hi c_nzuimm6lo": "(c.rdrs1n0(), c.c_nzuimm6())",  # c.slli
    # F-extension.
    "rd rs1 rs2 rs3 rm": "(c.rd(), c.rs1(), c.rs2(), c.rs3(), c.rm())",
    "rd rs1 rs2 rm": "(c.rd(), c.rs1(), c.rs2(), c.rm())",
    "rd rs1 rm": "(c.rd(), c.rs1(), c.rm())",
}


@dataclass
class BitPattern:
    hi: int
    lo: int
    value: int


@dataclass
class Spec:
    operator: str
    operands: Tuple[str]
    patterns: Tuple[BitPattern]


def parse_pattern(pattern: str) -> BitPattern:
    # Bit patterns consist of a bit range "hi..lo" or "bit", an "=" sign, then the value of those bits.
    bit_range, value = pattern.split("=")

    # Parse the bit-range. It's either hi..lo or a single bit.
    (hi, lo) = (
        bit_range.split("..") if ".." in bit_range else (bit_range, bit_range)
    )
    hi, lo = int(hi), int(lo)

    # Parse the value. It can be any base, hence the 0.
    value = int(value, 0)

    bit_pattern = BitPattern(hi, lo, value)

    return bit_pattern


def parse_spec(line: str) -> Spec:
    # Split each line into pieces.
    pieces = line.split()

    # Each line starts with an operator, followed by operands and bit patterns.
    operator, *rest = pieces

    # Operands do not contain an "=" sign.
    operands = tuple(item for item in rest if "=" not in item)

    # Bit patterns consist of a bit range "hi..lo", an "=" sign, then the value of those bits.
    bit_patterns = tuple(parse_pattern(item) for item in rest if "=" in item)

    spec = Spec(operator, operands, bit_patterns)

    return spec


def parse(spec: str) -> List[Spec]:
    # Remove comments and blank lines.
    lines = [line.strip() for line in spec.splitlines()]
    lines = [line for line in lines if not len(line) == 0 and line[0] != "#"]

    # Extract a spec for each line.
    specs = [parse_spec(line) for line in lines]
    return specs


def generate_naive_code(specs: List[Spec]):
    """
    This is the baseline "naive" code generator that creates a Rust `if` expression for each spec that it sees.
    """
    # For each spec, generate a Rust `if` expression that tests the bit-ranges for the values and invokes a handler if
    # they match.
    for spec in specs:
        condition = " && ".join(
            f"c.bits({pattern.hi}, {pattern.lo}) == {pattern.value}"
            for pattern in spec.patterns
        )
        operator = spec.operator.replace(".", "_")
        operands = lut.get(" ".join(spec.operands), "()")
        expression = f"if {condition} {{ return decoder.{operator}{operands} }}"
        print(expression)


def show_bit_patterns(specs: List[Spec]):
    from pprint import pprint
    all_patterns = []
    for spec in specs:
        patterns = tuple((pattern.hi, pattern.lo) for pattern in spec.patterns)
        all_patterns.append(patterns)
    print(f"There are {len(all_patterns)} total patterns")
    pprint(all_patterns)
    print(f"There are {len(set(all_patterns))} unique patterns")
    pprint(set(all_patterns))


def count_bits(patterns: List[Tuple]) -> int:
    bits = 0
    for hi, lo in patterns:
        bits += 1 + (hi - lo)
    return bits


def generate_matching_code(specs: List[Spec]):
    """Generates a decoder that uses Rust match expressions."""
    all_patterns = dict()
    for spec in specs:
        patterns = tuple((pattern.hi, pattern.lo) for pattern in spec.patterns)
        value = all_patterns.get(patterns, [])
        values = (tuple(pattern.value for pattern in spec.patterns), spec.operator, spec.operands)
        value.append(values)
        all_patterns[patterns] = value
    sorted_keys = sorted(all_patterns.keys(), key=lambda x: count_bits(x), reverse=True)
    ordered_patterns = dict()
    for k in sorted_keys:
        ordered_patterns[k] = all_patterns[k]
    for k, v in ordered_patterns.items():
        match_operands = ", ".join([f"c.bits{bits}" for bits in k])
        print(f"match ({match_operands}) {{")
        for value, operator, operands in v:
            match_condition = ", ".join(str(n) for n in value)
            operator = operator.replace(".", "_")
            operands = lut.get(" ".join(operands), "()")
            print(f"({match_condition}) => return decoder.{operator}{operands},")
        print("_ => {}")
        print("}")


def make_bitmask(bits) -> int:
    mask = 0
    for hi, lo in bits:
        for n in range(lo, hi+1):
            mask = mask | (1 << n)
    return mask


def make_bitpattern(bits, values) -> int:
    result = 0
    for (_, lo), v in zip(bits, values):
        result = result | (v << lo)
    return result


def generate_bitmask_code(specs: List[Spec], extensions: str):
    """Generates a decoder that uses Rust match expressions based on bitmasks."""

    patterns = "\n        + ".join(f"DecodeRv32{x}\n        + DecodeRv32{x}<Item = U>" for x in extensions)
    preamble = f"""\
pub fn decode<T, U>(decoder: &mut T, code: u32) -> U
where
    T: {patterns},
{{
    // This function is generated by make_decoder.py. Do not edit.
    let c = ToBits(code);
"""

    postamble = """\
    decoder.illegal(code)
}
"""

    all_patterns = dict()
    for spec in specs:
        patterns = tuple((pattern.hi, pattern.lo) for pattern in spec.patterns)
        value = all_patterns.get(patterns, [])
        values = (tuple(pattern.value for pattern in spec.patterns), spec.operator, spec.operands)
        value.append(values)
        all_patterns[patterns] = value
    sorted_keys = sorted(all_patterns.keys(), key=lambda x: count_bits(x), reverse=True)
    ordered_patterns = dict()
    for k in sorted_keys:
        ordered_patterns[k] = all_patterns[k]

    print(preamble)        
    for k, v in ordered_patterns.items():
        mask = make_bitmask(k)
        if mask != 0xffffffff:
            print(f"    match code & 0x{mask:08x} {{")
        else:
            print("    match code {")
        for value, operator, operands in v:
            match_value = make_bitpattern(k, value)
            operator = operator.replace(".", "_")
            operands = lut.get(" ".join(operands), "()")
            width = 8 if (match_value & 3) == 3 else 4
            print(f"        0x{match_value:0{width}x} => return decoder.{operator}{operands},")
        print("        _ => {}")
        print("    }")
    print(postamble)


def parse_command_line():
    parser = argparse.ArgumentParser(description="Generate a RISC-V decoder for the RV32I base ISA plus extensions.")
    parser.add_argument("-c", dest="extensions", help="Enable the 'C' extension", action="append_const", const="c")
    parser.add_argument("-f", dest="extensions", help="Enable the 'F' extension", action="append_const", const="f")
    parser.add_argument("-m", dest="extensions", help="Enable the 'M' extension", action="append_const", const="m")
    args = parser.parse_args()
    args.extensions = "i" + "".join(sorted(list(set(args.extensions)) if args.extensions is not None else []))
    return args

if __name__ == "__main__":
    args = parse_command_line()

    decoders = dict(
        i=rv32i,
        c=rv32c,
        f=rv32f,
        m=rv32m,
    )

    opcodes_to_parse = "\n".join(decoders[x] for x in args.extensions)

    specs = parse(opcodes_to_parse)
    # generate_naive_code(specs)
    # generate_matching_code(specs)

    generate_bitmask_code(specs, args.extensions)
