enum Instruction {
    // opcode: 0000 0000, does nothing
    Nop,
    // opcode: 00 +(16b register)+ 0001, load next 2 bytes as 16b immidiate into r16
    Ld16im { reg: u8, imm: u16 },
    // opcode: 00 +[16b register]+ 0010, load value of a into memory location pointed by [r16]
    Ld16a { reg: u8 },
    // opcode: 00 +(16b register)+ 1010, load value of memory location pointed by [r16] into a
    Lda16 { reg: u8 },
    // opcode: 0000 1000, load value of SP & $FF into memory location pointed by next 3 bytes as 16b immidiate(n16) and SP >> 8 at memory location n16 + 1.
    Ldn16sp { imm: u16 },

    // opcode: 00 +(16b register)+ 0011, increments r16
    Inc16 { reg: u8 },
    // opcode: 00 +(16b register)+ 1011, decrements r16
    Dec16 { reg: u8 },
    // opcode: 00 +(16b register)+ 0001, adds r16 to hl
    Addhl16 { reg: u8 },

    // opcode: 00 +(8b register)+ 100, increments r8
    Inc8 { reg: u8 },
    // opcode: 00 +(8b register)+ 101, decrements r8
    Dec8 { reg: u8 },
    // opcode 00 +(8b register)+ 110, loads next byte as 8b immidiate into r8
    Ld8im { reg: u8, imm: u8 },

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
}

// six 8bit registers (b, c, d, e, h, l, a)
// organisation: 000 = b, 001 = c, 010 = d, 011 = e, 100 = h, 101 = l , 110 = [hl], 111 = a
// 4 16bit registers (bc, de, hl, sp)
// organisation: 00 = bc, 01 = de, 10 = hl, 11 = sp (stack pointer)
// 4 16bit pointers ([bc], [de], [hl+], [hl-]) respectively 00 01 10 11
