use lc3b_isa::Instruction;

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn from_assembly(program: &str) -> Result<Program, crate::Error> {
        let instructions = lc3b_assembler::parse_to_program(program)
            .map_err(|e| crate::Error::ParseAssembly(format!("{:?}", e)))?;
        Ok(Program { instructions })
    }
}
