# RISC-Z
A RISC architecture for a virtual machine.

The machine has 256 pages of memory that each have 256 addresses, giving a total of 65 kilobytes of addressable memory. It has 15 8-bit registers and a zero registter (a register that discards the value put into it and always contains zero).
<p align="center"><img width="512" height="384" alt="r0" style="text-align: center" src="https://github.com/user-attachments/assets/6a76d116-859d-4134-a6c5-c22c0bc39f65" /></p>


### RISC style architectures
A Reduced Instruction Set Computer (RISC) ascribes to the theory that a computer should have a limited number of extremely quick instructions. This is opposed to Complex Instruction Set Computers (CISCs) that implement many different instructions for specific uses. In accordance with this belief, RISC machines often use Load-Store Architecture. In this architecture, the machine loads values into general purpose registers and performs operations on them. They then store the values back into memory. This means that instructions only perform operations on the registers.

### Details
You can view more details about the architecture of this project [here](./ISA.md)

## Installation
This project is meant to work with the [RISC-Z assembler](https://github.com/slicetext/RISCZ-asm). Both of these projects are built with cargo.
```bash
git clone github.com/slicetext/RISC-Z.git
git clone github.com/slicetext/RISCZ-asm.git

cd RISC-Z && cargo install & cd ..
cd RISCZ-asm && cargo install & cd..
```
## Usage
Run `cargo run -- filename`.
