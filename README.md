# arviss

Arviss is ***A*** ***R***ISC-***V*** ***I***nstruction ***S***et ***S***imulator for 32-bit variants of the [RISC-V](https://en.wikipedia.org/wiki/RISC-V) instruction set architecture ([ISA](https://en.wikipedia.org/wiki/Instruction_set_architecture)).

It provides a toolkit for creating software implementations of the base 32-bit variant of the RISC-V ISA and a number of its extensions, including:

- the RV32I base integer instruction set
- the ‘M’ standard extension for integer multiplication and division
- the ‘F’ standard extension for single-precision floating point
- the ‘C’ standard extension for compressed instructions

# Building

## Pre-requisites

[Install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html). This is a link to the docs that give you some sensible choices of how to install Rust. I strongly recommend that you do *not* run the convenient yet highly insecure method of piping the output of `curl` directly into `sh`.

## Build the simulator

### Clone this repository.

```
$ git clone https://github.com/badlydrawnrod/arviss_experiment
```

### Perform a release build.

```
$ cd arviss_experiment
$ cargo build --release
```

## Run the tests

```
$ cargo test
```

## Build the documentation

```
$ cargo doc --open
```

# Running
## Run the examples

### Run `hello_world`

This loads an image and executes it with an RV32I CPU.
```
$ cargo run --example hello_world
```
You should see output that looks like this.
```
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
```

### Run `disassemble_hello_world`

This loads an RV32IC image and disassembles it to stdout.
```
$ cargo run --example disassemble_hello_world
```
You should see output that looks like this.
```
addr     instr    code
00000000 00005197 auipc gp, 5
00000004 80018193 addi  gp, gp, -2048
00000008 00008117 auipc sp, 8
0000000c ff810113 addi  sp, sp, -8
00000010     840a add   s0, zero, sp
00000012 00004517 auipc a0, 4
...
0000020a     4b12 lw    s6, 4(sp)
0000020c     6105 addi  sp, sp, 32
0000020e     8082 jalr  zero, ra, 0
```

### Run `runner`

This loads, runs and optionally traces an RV32I image passed on the command line.
```
$ cargo run --example runner images/hello_world.rv32i
```
You should see output that looks like this.
```
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
Hello, world from Rust!
```
If you try the same with an RV32IC image then it will fail with an illegal instruction,
```
$ cargo run --example runner -- -d images/hello_world.rv32ic
```
You should see output that looks like this.
```
pc       (pc)     Code
00000000 00005197 auipc gp, 5
00000004 80018193 addi  gp, gp, -2048
00000008 00008117 auipc sp, 8
0000000c ff810113 addi  sp, sp, -8
00000010 0000840a illegal instruction: 840a
IllegalInstruction(33802) at 0x00000010
```
