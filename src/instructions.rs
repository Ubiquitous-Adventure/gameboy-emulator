pub enum Instruction {
    // opcode: 0000 0000, no operation
    Nop,
    // simple load operations
    // opcode: 00 +(16b register)+ 0001, load next 2 bytes as 16b immidiate into r16
    Ld16im { reg: u8, imm: u16 },
    // opcode: 00 +[16b register]+ 0010, load value of a into memory location pointed by [r16]
    Ld16a { reg: u8 },
    // opcode: 00 +(16b register)+ 1010, load value of memory location pointed by [r16] into a
    Lda16 { reg: u8 },
    // opcode: 0000 1000, load value of SP & $FF into memory location pointed by next 3 bytes as 16b immidiate(n16) and SP >> 8 at memory location n16 + 1.
    Ldrn16sp { imm: u16 },

    // opcode: 00 +(16b register)+ 0011, increments r16
    Inc16 { reg: u8 },
    // opcode: 00 +(16b register)+ 1011, decrements r16
    Dec16 { reg: u8 },
    // opcode: 00 +(16b register)+ 1001, adds r16 to hl
    Addhl16 { reg: u8 },
    // opcode: 00 +(8b register)+ 100, increments r8
    Inc8 { reg: u8 },
    // opcode: 00 +(8b register)+ 101, decrements r8
    Dec8 { reg: u8 },
    // opcode: 00 +(8b register)+ 110, loads next byte as 8b immidiate into r8
    Ld8im { reg: u8, imm: u8 },

    // opcode: 0000 0111, rotate a left
    Rlca,
    // opcode: 0000 1111, rotate a right
    Rrca,
    // opcode: 0001 0111, rotate a left (through the carry)
    Rla,
    // opcode: 0001 1111, rotate a right (through the carry)
    Rra,
    // opcode: 0010 0111, adjusts a to be a correct bdc interpretation by adding 6 to the respective nibble (4 low or high bits) and adjusting the carry
    Daa,
    // opcode: 0010 1111, complements a bitwise
    Cpl,
    // opcode: 0011 0111, sets carry flag
    Scf,
    // opcode: 0011 1111, complements carry flag
    Ccf,

    // opcode 0001 1000, jumps relatively using next byte as 8b immidiate
    Jr { imm: u8 },
    // opcode 001 +(condition)+ 000, jumps relatively using next byte as 8b immidiate if condition is true
    Jrcond { cond: u8, imm: u8 },
    // opcode 0001 0000, stops and enters low power mode, depending on next byte behaviour may vary (TODO)
    Stop { next: u8 },

    // opcode: 01 +(8b register dest)+(8b register source), loads value of source r8 into destination r8
    Ld88 { dest: u8, source: u8 },
    // exception: 0111 0110, loading [hl] into [hl] instead halts (check documentation on halt for more info)
    Halt,

    // 8 bit arithmatic with 8b register
    // opcode: 1000 0 +(8b register), adds value in r8 to a
    Adda { reg: u8 },
    // opcode: 1000 1 +(8b register), adds value in r8 + carry to a
    Adca { reg: u8 },
    // opcode: 1001 0 +(8b register), subtracts value in r8 from a
    Suba { reg: u8 },
    // opcode: 1001 1 +(8b register), subtracts value in r8 + carry from a
    Sbca { reg: u8 },
    // opcode: 1010 0 +(8b register), bitwise and with value in r8 and a
    Anda { reg: u8 },
    // opcode: 1010 1 +(8b register), bitwise xor with value in r8 and a
    Xora { reg: u8 },
    // opcode: 1011 0 +(8b register), bitwise or with value in r8 and a
    Ora { reg: u8 },
    // opcode: 1011 1 +(8b register), subtracts value in r8 from a without saving (compare)
    Cpa { reg: u8 },

    // 8bit arithmatic with immidiate
    // opcode: 1100 0110, adds next byte as 8b immidiate to a
    Addim { imm: u8 },
    // opcode: 1100 1110, adds next byte as 8b immidiate + carry to a
    Adcim { imm: u8 },
    // opcode: 1101 0110, subtracts next byte as 8b immidiate from a
    Subim { imm: u8 },
    // opcode: 1101 1110, subtracts next byte as 8b immidiate + carry from a
    Sbcim { imm: u8 },
    // opcode: 1110 0110, bitwise and with next byte as 8b immidiate and a
    Andim { imm: u8 },
    // opcode: 1110 1110, bitwise xor with next byte as 8b immidiate and a
    Xorim { imm: u8 },
    // opcode: 1111 0110, bitwise or with next byte as 8b immidiate and a
    Orim { imm: u8 },
    // opcode: 1111 1110, adds next byte as 8b immidiate to a without saving (compare)
    Cpim { imm: u8 },

    // return and jump instructions
    //opcode: 110 +(condition)+ 000, returns from subroutine if condition is true
    Retc { cond: u8 },
    //opcode: 11001001, returns from subroutine (using stack value)
    Ret,
    //opcode: 11011001, returns subroutine and sets IME enabeling interrups (EI into RET)
    Reti,
    //opcode: 110 +(condition)+ 010, loads next 2 bytes as 16b immdiate into pc to jump if condtion is true
    Jpcim { cond: u8, imm: u16 },
    //opcode: 11000011, loads next 2 bytes as 16b immdiate into pc to jump
    Jpim { imm: u16 },
    //opcode: 11101001,, loads value of hl register into pc to jump
    Jphl,
    //opcode: 110 +(condition)+ 100, pushes next 2 bytes as 16b immidiate on stack (so RET can later pop it for called jump)
    Callcim { cond: u8, imm: u16 },
    //opcode: 11001101, pushes next 2 bytes as 16b immidiate on stack (so RET can later pop it for called jump)
    Callim { imm: u16 },
    //opcode: 11 +(3b index)+ 111, pushes given vector adress on stack (so RET can later opo it for called jump)
    Rst { vec: u8 },
    //opcode: 11 +(16b stack reg)+ 0001, pops value of stack into 16b stack register (low then high)
    Pop16 { reg: u16 },
    //opcode: 11 +(16b stack reg)+ 0101, pushes value of 16b stack register on stack (high then low)
    Push16 { reg: u16 },

    // prefix instructions: opcode of prefix: 1100 1011
    // opcode: 0000 0 +(8b register), rotate r8 left
    Rlc { reg: u8 },
    // opcode: 0000 1 +(8b register), rotate r8 right
    Rrc { reg: u8 },
    // opcode: 0001 0 +(8b register), rotate r8 left (through the carry)
    Rl { reg: u8 },
    // opcode: 0001 1 +(8b register), rotate r8 right (through the carry)
    Rr { reg: u8 },
    // opcode: 0010 0 +(8b register), shift r8 left arithmetically (with 0)
    Sla { reg: u8 },
    // opcode: 0010 1 +(8b register), shift r8 right arithmetically (doesnt change b7)
    Sra { reg: u8 },
    // opcode: 0011 0 +(8b register), swap upper 4 bits with lower 4 bits of r8
    Swap { reg: u8 },
    // opcode: 0011 1 +(8b register), shift r8 right logically (with 0)
    Srl { reg: u8 },
    // opcode 01 +(3b index)+(8b register), test if bit at index in r8 is set (z flag)
    Bit { index: u8, reg: u8 },
    // opcode 10 +(3b index)+(8b register), set bit at index in r8 to 0
    Res { index: u8, reg: u8 },
    // opcode 11 +(3b index)+(8b register), set bit at index in r8 to 1
    Set { index: u8, reg: u8 },

    // short cut codes
    //opcode: 11100010, loads value of a into adress at FF00 + value in c
    Ldhca,
    //opcode: 11100000, loads value of a into adress at FF00 + imm8 (next byte as 8b immidiate)
    Ldh8a { imm: u8 },
    //opcode: 11101010, loads value of a into adress at location [imm16] (next 2byte as 16b immidiate)
    Ldr16a { imm: u16 },
    //opcode: 11110010, loads value at adress FF00 + c into a
    Ldhac,
    //opcode: 11110000, loads value of adress FF00 + imm8 (next byte as 8b immidiate) into a
    Ldha8 { imm: u8 },
    //opcode: 11111010, loads value of adress at location [imm16] (next 2byte as 16b immidiate) into a
    Ldar16 { imm: u16 },
    //opcode: 11101000, adds next byte as 8b immidiate to sp
    Addsp8 { imm: u8 },
    //opcode: 11111000, loads value of sp + next byte as 8b immidiate into hl
    Ldhlsp8 { imm: u8 },
    //opcode: 11111001, loads value of hl into sp
    Ldsphl,
    //opcode: 11110011, resets IME to disable interrupts
    Di,
    //opcode: 11111011, sets IME to enable interrupts
    Ei,

    // all the following opcodes are invalid and hard lock cpu by never fetching (just exit in this case)
    // 1101 0011, 1101 1011, 1101 1101, 1110 0011, 1110 0100, 1110 1011, 1110 1100, 1110 1101, 1111 0100, 1111 1100, 1111 1101
    Inv,

    // extra instructions for constants that can be used as place holders
    Const { val: u8 },
}

// six 8bit registers (b, c, d, e, h, l, a)
// organisation: 000 = b, 001 = c, 010 = d, 011 = e, 100 = h, 101 = l , 110 = [hl], 111 = a
// 4 16bit registers (bc, de, hl, sp)
// organisation: 00 = bc, 01 = de, 10 = hl, 11 = sp (stack pointer)
// refering to 16bit stack registers changes sp to af (high is a, low if flag register )
// stack: classic stack data structure with one byte for each layer with sp as stackpointer

// 4 16bit pointers ([bc], [de], [hl+], [hl-]) respectively 00 01 10 11

// 4 different conditions depending on the zero and carry flags: (nz, z, nc, c) respectively 00 01 10 11

// rst vectors are addresses (in hex) 00, 08, 10, 18, 20, 28, 30, 38
// it takes a 3bit number then divides with 8 to directly adress one of them

// note: be careful with flags on each operation
