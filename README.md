# arviss_experiment

Arviss as a rewrite-it-in-Rust experiment.

# Building

## Install the prerequisites

- Install and build the [simple RISC-V runtime](https://github.com/badlydrawnrod/rt) and its prerequisites as described in its [README](https://github.com/badlydrawnrod/rt/blob/master/README.md).

- Install and build [the sample application](https://github.com/badlydrawnrod/rt_app)

## Build the simulator

- `git clone` this code into a parallel directory

```
$ git clone  clone https://github.com/badlydrawnrod/arviss_execute
```

- Perform a releae build

```
$ cd arviss_execute
$ cargo build --release
```

## Running

Run the simulator, passing it the path to the `app.bin` that you generated when you build [the sample application](https://github.com/badlydrawnrod/rt_app).

```
$ cargo run --release --bin runner ../rt_app/app.bin
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

If you want to disassemble each instruction as it is executed then supply the `-d` flag before the filename.

```
$ cargo run --release --bin runner -- -d ../rt_app/app.bin
```
