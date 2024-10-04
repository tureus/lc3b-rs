#![allow(dead_code)]

use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AddInstruction {
    AddReg(Register, Register, Register),
    AddImm(Register, Register, Immediate5),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AndInstruction {
    AndReg(Register, Register, Register),
    AndImm(Register, Register, Immediate5),
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl FromStr for Register {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = match s {
            "r0" | "R0" => Register::Register0,
            "r1" | "R1" => Register::Register1,
            "r2" | "R2" => Register::Register2,
            "r3" | "R3" => Register::Register3,
            "r4" | "R4" => Register::Register4,
            "r5" | "R5" => Register::Register5,
            "r6" | "R6" => Register::Register6,
            "r7" | "R7" => Register::Register7,
            unknown => return Err(eyre::eyre!("unhandled register identifier: {}", unknown)),
        };

        Ok(reg)
    }
}

impl Register {
    pub fn to_index(&self) -> usize {
        match *self {
            Register::Register0 => 0,
            Register::Register1 => 1,
            Register::Register2 => 2,
            Register::Register3 => 3,
            Register::Register4 => 4,
            Register::Register5 => 5,
            Register::Register6 => 6,
            Register::Register7 => 7,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Immediate5(pub u8);

impl FromStr for Immediate5 {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: range check
        Ok(Immediate5(s.parse()?))
    }
}

impl Immediate5 {
    pub fn to_value(&self) -> u16 {
        self.0 as u16
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Immediate4(u8);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Condition {
    n: bool,
    z: bool,
    p: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PCOffset9(u16);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PCOffset11(u16);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PCOffset6(u8);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bit(bool);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TrapVect8(u8);
