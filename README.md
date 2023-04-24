# arviss_experiment

Arviss as a rewrite-it-in-Rust experiment.

# Building

## Pre-requisites

[Install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html). This is a link to the docs that give you some sensible choices of how to install Rust. I strongly recommend that you do *not* run the convenient yet highly insecure method of piping the output of `curl` directly into `sh`.

## Build the simulator

### Clone this repository.

```
$ git clone https://github.com/badlydrawnrod/arviss_execute
```

### Perform a release build.

```
$ cd arviss_execute
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
$ cargo run --bin hello_world
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
$ cargo run --bin disassemble_hello_world`
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
