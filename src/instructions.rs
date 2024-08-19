#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R8Operand {
    AReg,
    BReg,
    CReg,
    DReg,
    EReg,
    HReg,
    LReg,
    HLAddr,
}

impl From<u8> for R8Operand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BReg,
            1 => Self::CReg,
            2 => Self::DReg,
            3 => Self::EReg,
            4 => Self::HReg,
            5 => Self::LReg,
            6 => Self::HLAddr,
            7 => Self::AReg,
            _ => unreachable!("R16 operands must be 3 bits"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16Operand {
    BCReg,
    DEReg,
    HLReg,
    SP,
}

impl From<u8> for R16Operand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BCReg,
            1 => Self::DEReg,
            2 => Self::HLReg,
            3 => Self::SP,
            _ => unreachable!("R16 operands must be 2 bits"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16StkOperand {
    BCReg,
    DEReg,
    HLReg,
    AFReg,
}

impl From<u8> for R16StkOperand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BCReg,
            1 => Self::DEReg,
            2 => Self::HLReg,
            3 => Self::AFReg,
            _ => unreachable!("R16 stack operands must be 2 bits"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16MemOperand {
    BCReg,
    DEReg,
    HLRegAndInc,
    HLRegAndDec,
}

impl From<u8> for R16MemOperand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BCReg,
            1 => Self::DEReg,
            2 => Self::HLRegAndInc,
            3 => Self::HLRegAndDec,
            _ => unreachable!("R16 memory operands must be 2 bits"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondOperand {
    NZ,
    Z,
    NC,
    C,
}

impl From<u8> for CondOperand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NZ,
            1 => Self::Z,
            2 => Self::NC,
            3 => Self::C,
            _ => unreachable!("Condition operands must be 2 bits"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum U3Operand {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

impl From<u8> for U3Operand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            _ => unreachable!("U3 operands must be 3 bits"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// nop - do nothing
    Nop,
    /// ld r16, imm16 - load 16-bit immediate into 16-bit register
    LoadImm16 { reg: R16Operand, imm: u16 },
    /// ld [r16mem], a - store 8-bit value from A register into byte pointed to by 16-bit register
    StoreARegToMem { mem: R16MemOperand },
    /// ld a, [r16mem] - load 8-bit value from byte pointed to by 16-bit register into A register
    LoadMemToAReg { mem: R16MemOperand },
    /// ld [imm16], sp - store 16-bit stack pointer into the two bytes pointed to by immediate
    StoreSPToImmMem { imm: u16 },
    /// inc r16 - increment 16-bit register
    IncR16 { reg: R16Operand },
    /// dec r16 - decrement 16-bit register
    DecR16 { reg: R16Operand },
    /// add hl, r16 - add value from 16-bit register to HL register
    AddRegToHLReg { reg: R16Operand },
    /// inc r8 - increment 8-bit register
    IncR8 { reg: R8Operand },
    /// dec r8 - decrement 8-bit register
    DecR8 { reg: R8Operand },
    /// ld r8, imm8 - load 8-bit immediate into 8-bit register
    LoadImm8 { dst: R8Operand, imm: u8 },
    /// rlca - rotate A register left
    RotARegLeftSetC,
    /// rrca - rotate A register left
    RotARegRightSetC,
    /// rla - rotate A register left through the carry floag
    RotARegLeftThroughC,
    /// rra - rotate A register right through the carry floag
    RotARegRightThroughC,
    /// daa - decimal adjust accumulator to get correct BCD representation
    DecAdjAccum,
    /// cpl - invert A register
    InvA,
    /// scf - set carry flag
    SetC,
    /// ccf - invert carry flag
    InvC,
    /// jr imm8 - jump to address with signed 8-bit immediate offset
    JumpRelativeImm { imm: i8 },
    /// jr cond, imm8 - jump to address with signed 8-bit immediate offset if condition is met
    JumpRelativeImmUnderCond { cond: CondOperand, imm: i8 },
    /// stop - do nothing but is (often) considered a two-byte instruction
    Stop,
    /// ld r8dst, r8src - load value from 8-bit register into another 8-bit register
    LoadR8ToR8 { dst: R8Operand, src: R8Operand },
    /// halt - enter CPU low-power mode until interrupt occurs
    Halt,
    /// add a, r8 - add value from 8-bit register to the A register
    AddRegToAReg { reg: R8Operand },
    /// adc a, r8 - add value from 8-bit register plus the carry flag to the A register
    AddRegCToAReg { reg: R8Operand },
    /// sub a, r8 - subtract 8-bit register from the A register
    SubRegFromAReg { reg: R8Operand },
    /// sbc a, r8 - subtract 8-bit register and the carry flag from the A register
    SubRegCFromAReg { reg: R8Operand },
    /// and a, r8 - bitwise and between 8-bit register and the A register
    AndRegToAReg { reg: R8Operand },
    /// xor a, r8 - bitwise xor between 8-bit register and the A register
    XorRegToAReg { reg: R8Operand },
    /// or a, r8 - bitwise or between 8-bit register and the A register
    OrRegToAReg { reg: R8Operand },
    /// cp a, r8 - compare 8-bit register and A register by substracting and setting flags
    CmpRegToAReg { reg: R8Operand },
    /// add a, imm8 - add 8-bit immediate to the A register
    AddImmToAReg { imm: u8 },
    /// adc a, imm8 - add 8-bit immediate plus the carry flag to the A register
    AddImmCToAReg { imm: u8 },
    /// sub a, imm8 - subtract 8-bit immediate from the A register
    SubImmFromAReg { imm: u8 },
    /// sbc a, imm8 - subtract 8-bit immediate and the carry flag from the A register
    SubImmCFromAReg { imm: u8 },
    /// and a, imm8 - bitwise and between 8-bit register and the A register
    AndImmToAReg { imm: u8 },
    /// xor a, imm8 - bitwise xor between 8-bit register and the A register
    XorImmToAReg { imm: u8 },
    /// or a, imm8 - bitwise or between 8-bit register and the A register
    OrImmToAReg { imm: u8 },
    /// cp a, imm8 - compare 8-bit register and 8-bit immediate by substracting and setting flags
    CmpImmToAReg { imm: u8 },
    /// ret cond - return from subroutine if condition is met
    RetUnderCond,
    /// ret - return from subroutine (Pop PC)
    Ret,
    /// reti - return from subroutine and enable interrupts
    RetInterrupts,
    /// jp cond, imm16 - jump to 16-bit immediate address if condition is met
    JumpImmUnderCond,
    /// jp imm16 - jump to 16-bit immediate address
    JumpImm,
    /// jp hl - jump to 16-bit address stored in HL register
    JumpHL,
    /// call cond, imm16 - call 16-bit immediate if condition is met
    CallImmUnderCond { cond: CondOperand, imm: u16 },
    /// call imm16 - call 16-bit immediate address
    CallImm { imm: u16 },
    /// rst tgt3 - call address tgt3 * 8
    CallRst { target: U3Operand },
    /// pop r16stk - pop 16-bit register from the stack
    Pop { reg: R16Operand },
    /// push r16stk - push 16-bit register to the stack
    Push { reg: R16Operand },
    /// ldh [c], a - store 8-bit value from A register to memory at 0xFF00 + C
    StoreARegToCMem,
    /// ldh [imm8], a - store 8-bit value from A register to memory at 0xFF00 + 8-bit immediate
    StoreARegToImm8Mem { imm: u8 },
    /// ld [imm16], a - store 8-bit value from A register to memory pointed to by 16-bit immediate
    StoreARegToImm16Mem { imm: u16 },
    /// ldh a, [c] - load 8-bit value from 0xFF00 + C into A register
    LoadCMemToAReg,
    /// ldh a, [imm8] - load 8-bit value from 0xFF00 + 8-bit immediate into A register
    LoadImm8MemToAReg { imm: u8 },
    /// ld a, [imm16] - load 8-bit value from memory at 16-bit immediate address into A register
    LoadImm16MemToAReg { imm: u16 },
    /// add sp, imm8 - add a signed 8-bit immediate to the stack pointer
    AddImmToSP { imm: i8 },
    /// ld hl, sp + imm8 - add signed 8-bit immediate to the stack pointer and save in HL
    LoadSPWithImmToHLReg { imm: i8 },
    /// ld sp, hl - load 16-bit register from HL register into stack pointer
    LoadHLRegToSP,
    /// di - disable interrupts by clearing the IME flag
    DisableInterrupts,
    /// ei - enable interrupts by setting the IME flag
    EnableInterrupts,
    /// rlc r8 - rotate 8-bit register left
    RotR8LeftSetC,
    /// rrc r8 - rotate 8-bit register right
    RotR8RightSetC,
    /// rl r8 - rotate 8-bit register left through the carry flag
    RotR8LeftThroughC,
    /// rr r8 - rotate 8-bit register right through the carry flag
    RotR8RightThroughC,
    /// sla r8 - arithmetically shift left 8-bit register
    ShiftLeftArith { reg: R8Operand },
    /// sra r8 - arithmetically shift right 8-bit register
    ShiftRightArith { reg: R8Operand },
    /// swap r8 - swap the upper 4 bits in 8-bit register with the lower 4 bits
    SwapHighLowR8 { reg: R8Operand },
    /// srl r8 - logically shift right 8-bit register
    ShiftLeftLogic { reg: R8Operand },
    /// bit b3, r8 - test b3-th bit in 8-bit register
    TestBit { bit_num: U3Operand, reg: R8Operand },
    /// res b3, r8 - set b3-th bit in 8-bit register to zero
    SetBitZero { bit_num: U3Operand, reg: R8Operand },
    /// set b3, r8 - set b3-th bit in 8-bit register to one
    SetBitOne { bit_num: U3Operand, reg: R8Operand },
}
