# ISA  
Memory: 8 bit addresses; size of 8 bit values  
Registers: 16 8 bit registers; r1..rF; r0 is zero register, rF is used by pseudo instructions as auxiliary memory  
Instruction memory: 12 bit addresses, 16 bit instructions   
Call stack: Size of 64 8 bit addresses  
Flags: result flag, memory page (u8)
Instructions (r1..f are registers, a1.. are instruction addresses, m1.. are memory addresses, v1.. are raw 8 bit values):  

| Instruction | Operation |
| ----------------------- | ----------------------------------------------------------- |
| ADD 0000 r1 r2 r3 | Add r2 and r3, result into r1 |
| SUB 0001 r1 r2 r3 | Subtract r3 from r2, store in r1 |
| DIV 0010 r1 r2 r3  | Divide r2 by r3, store in r1 |
| AND 0011 r1 r2 r3 | Bitwise and with r2 and r3, store in r1 |
| ORR 0100 r1 r2 r3 | Bitwise or with r2 and r3, store in r1 |
| XOR 0101 r1 r2 r3 | Bitwise xor with r2 and r3, store in r1 |
| NOT 0110 r1 r2 XXXX | Store not r2 in r1 |
| LSH 0111 r1 r2 r3 | Bitwise left shift r2 by r3 places, store in r1 |
| RSH 1000 r1 r2 r3 | Bitwise right shift r2 by r3 places, store in r1 |
| RET 1001 XXXX XXXX XXXX | Return to address at top of call stack, pop from call stack |
| BIR 1010 a1 | If result flag is set jump to a1, push a1 to call stack |
| LDM 1011 r1 r2 | Load m\[r2\] in current memory page into r1 |
| STR 1100 r1 r2 | Store value of r2 in m\[r1\] in current memory page |
| LDI 1101 r1 v1 | Load v1 into r1 |
| CMP 1110 r1 r2 r3 | Compare r2 and r3 using the comparison provided in r1, set result flag |
| SPG 1111 r1 XXXX XXXX | Set memory page to r1 |
  
Comparisons
| Value | Comparison |
| - | - |
| 0 | equal |
| 1 | greater than |
| 2 | less than |
| 3 | greater than or equal |
| 4 | less than or equal |
| 5 | not equal |

Pseudo Instructions  

| Instruction  | Operation                                                    |
| ------------ | ------------------------------------------------------------ |
| RUN NAME     | Call named subroutine (JMP NAME)                             |
| MUL r1 r2 r3 | Multiply r2 by r3, store in r1 (Repeatedly run ADD r1 r2 r3) |
| JMP a1       | Jump to a1 (BIZ r0 a1)                                       |
| INC r1       | Increment r1 (LDI 1 rF; ADD r1 rF r1)                        |
| NOP | No operation, represented as ADD r0 r0 r0 |
  

