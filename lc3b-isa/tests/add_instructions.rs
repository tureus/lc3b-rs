use lc3b_isa::{AddInstruction, Instruction, Register};

#[test]
fn convert_add_inst_to_memory() {
    let inst = &Instruction::AddInstruction(AddInstruction::AddReg(
        Register::Register0,
        Register::Register1,
        Register::Register2,
    ));

    let inst_bytes: [u8; 2] = inst.into();
    assert_eq!([0b00010000, 0b01000010], inst_bytes);
}
