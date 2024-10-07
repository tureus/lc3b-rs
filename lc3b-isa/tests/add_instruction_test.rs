use lc3b_isa::{AddInstruction, Immediate5, Instruction, Register};

#[test]
fn add_three_register_to_bits() {
    let inst = &Instruction::AddInstruction(AddInstruction::AddReg(
        Register::Register0,
        Register::Register1,
        Register::Register2,
    ));

    let inst_bytes: [u8; 2] = inst.into();
    assert_eq!([0b00010000, 0b01000010], inst_bytes);
}

#[test]
fn add_imm_to_bits() {
    let inst = &Instruction::AddInstruction(AddInstruction::AddImm(
        Register::Register0,          // 0b00000000
        Register::Register1,          // 0b00000001
        Immediate5::new(31).unwrap(), // 0b00011111
    ));

    let inst_bytes: [u8; 2] = inst.into();
    let expected_bytes = [0b00010000, 0b01111111];
    assert_eq!(
        expected_bytes, inst_bytes,
        "\nexpected: {:08b}{:08b}\ngot:      {:08b}{:08b}",
        expected_bytes[0], expected_bytes[1], inst_bytes[0], inst_bytes[1]
    );
}
