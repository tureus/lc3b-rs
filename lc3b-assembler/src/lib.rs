#![forbid(unsafe_code)]

use std::{fmt::Debug, hash::Hash, str::FromStr};

use lc3b_isa::{AddInstruction, Immediate5, Instruction, Register};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(pest_derive::Parser)]
#[grammar = "lc3b_asm.pest"]
struct LC3BAsmParser {}

pub type Error = pest::error::Error<Rule>;

pub fn parse_to_pairs(program: &str) -> Result<Pairs<Rule>, Box<Error>> {
    LC3BAsmParser::parse(Rule::program, program).map_err(Box::new)
}

pub fn parse_to_program(program: &str) -> eyre::Result<Vec<Instruction>> {
    let program = LC3BAsmParser::parse(Rule::program, program)?
        .next()
        .unwrap();

    let instructions = program.into_inner();

    let instructions = instructions
        .filter_map(|i| match i.as_rule() {
            Rule::instruction => Some(instruction_from_pair(i)),
            Rule::comment => None,
            other => unimplemented!("don't handle {:#?}", other),
        })
        .collect::<Vec<_>>();

    if instructions.iter().any(Result::is_err) {
        return Err(eyre::eyre!(
            "there was an error parsing the program: {:#?}",
            instructions
        ));
    }

    let instructions = instructions.into_iter().flatten().collect();

    Ok(instructions)
}

fn instruction_from_pair(pair: Pair<Rule>) -> eyre::Result<Instruction> {
    let mut inner = pair.into_inner();
    let opcode = inner.next();
    if opcode.is_none() {
        return Err(eyre::eyre!("could not handle {:#?}", opcode));
    }
    let opcode = opcode.unwrap();

    let instruction = match opcode.as_str() {
        "ADD" => {
            let mut operands = inner.next().unwrap().into_inner();
            let arg_one = operands.next().unwrap().as_str();
            let dst_reg = Register::from_str(arg_one)?;

            let arg_two = operands.next().unwrap().as_str();
            let src_reg = Register::from_str(arg_two)?;

            let arg_three = operands.next().unwrap();
            let inner: AddInstruction = match arg_three.as_rule() {
                Rule::literal => {
                    let imm5 = Immediate5::from_str(arg_three.as_str())?;
                    AddInstruction::AddImm(dst_reg, src_reg, imm5)
                }
                Rule::register => {
                    let src2_reg = Register::from_str(arg_three.as_str())?;
                    AddInstruction::AddReg(src_reg, dst_reg, src2_reg)
                }
                _ => return Err(eyre::eyre!("unhandled `{:?}`", arg_three)),
            };
            Instruction::AddInstruction(inner)
        }
        other => panic!("unhandled opcode {:#?}", other),
    };

    Ok(instruction)
}

#[cfg(test)]
mod test {
    use lc3b_isa::{AddInstruction, Immediate5, Instruction, Register};

    #[test]
    pub fn stuff() {
        let test_asm = r#"
    ADD R1, R1, 8; this is a comment
    ADD R1, R2, 10;
"#;

        let instructions = super::parse_to_program(test_asm).unwrap();
        assert_eq!(
            instructions,
            [
                Instruction::AddInstruction(AddInstruction::AddImm(
                    Register::Register1,
                    Register::Register1,
                    Immediate5::new(8,).unwrap(),
                ),),
                Instruction::AddInstruction(AddInstruction::AddImm(
                    Register::Register1,
                    Register::Register2,
                    Immediate5::new(10,).unwrap(),
                ),),
            ]
        )
    }
}
