#[allow(dead_code)]
pub enum OpCode {
    ADD,
    AND,
    BR,
    JMP,
    JSR,
    LDB,
    LDI,
    LDR,
    LEA,
    NOT,
    RET,
    RTI,
    SHF,
    STB,
    STI,
    STR,
    TRAP
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::ADD  => 0b00000001,
            OpCode::AND  => 0b00000101,
            OpCode::BR   => 0b00000000,
            OpCode::JMP  => 0b00001100,
            OpCode::JSR  => 0b00000100,
            OpCode::LDB  => 0b00000010,
            OpCode::LDI  => 0b00001010,
            OpCode::LDR  => 0b00000110,
            OpCode::LEA  => 0b00001110,
            OpCode::NOT  => 0b00001001,
            OpCode::RET  => 0b00001100,
            OpCode::RTI  => 0b00001000,
            OpCode::SHF  => 0b00001101,
            OpCode::STB  => 0b00000011,
            OpCode::STI  => 0b00001011,
            OpCode::STR  => 0b00000111,
            OpCode::TRAP => 0b00001111,
        }
    }
}