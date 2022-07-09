#![allow(dead_code)]

pub enum Instruction {
    AddInstruction(AddInstruction),
    AndInstruction(AndInstruction),
    Br(Condition, PCOffset9),
    Jmp(Register),
    Jsr(PCOffset11),
    Jsrr(Register),
    Ldb(Register, Register, PCOffset6),
    Ldi(Register, Register, PCOffset6),
    Ldr(Register, Register, PCOffset6),
    Lea(Register, PCOffset9),
    Not(Register, Register),
    Ret,
    Rti,
    Shf(Register, Register, Bit, Bit, Immediate4),
    Stb(Register, Register, PCOffset6),
    Sti(Register, Register, PCOffset6),
    Str(Register, Register, PCOffset6),
    Trap(TrapVect8),
}

pub enum AddInstruction {
    AddReg(Register, Register),
    AddImm(Register, Immediate5),
}

pub enum AndInstruction {
    AndReg(Register, Register, Register),
    AndImm(Register, Register, Immediate5),
}

pub enum Register {
    Register0,
    Register1,
    Register2,
    Register3,
    Register4,
    Register5,
    Register6,
    Register7,
}

pub struct Immediate5(u8);
pub struct Immediate4(u8);

pub struct Condition {
    n: bool,
    z: bool,
    p: bool,
}

pub struct PCOffset9(u16);
pub struct PCOffset11(u16);
pub struct PCOffset6(u8);

pub struct Bit(bool);

pub struct TrapVect8(u8);
