# RISC-Z
A RISC architecture for a virtual machine.

The machine has 256 pages of memory that each have 256 addresses, giving a total of 65 kilobytes of addressable memory.

## Installation
This project is meant to work with the [RISC-Z assembler](https://github.com/slicetext/RISCZ-asm). Both of these projects are built with rust. `git clone` both repositories and build the projects with `cargo build`.

## Usage
Run `cargo run -- [filename]`.
