use lc3b_isa::{AddInstruction, Instruction, Register};
use wasm_bindgen::prelude::*;

use crate::wasm::{log, Callback};
use crate::Program;

pub struct CallbacksRegistry {
    pub hello: Callback,
}

impl CallbacksRegistry {
    pub fn call_hello(&self) {
        match self.hello {
            Callback::JS(ref function) => {
                let callback_response = function.call0(&JsValue::NULL);
                match callback_response {
                    Ok(_) => (),
                    Err(e) => {
                        crate::wasm::log(&format!("callback failed: `{:?}`", e));
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Computer {
    program: Program,
    program_counter: u16,
    condition: Condition,
    callbacks: CallbacksRegistry,
    registers: [u16; 8],
}

impl Computer {
    pub fn new(program: Program, callbacks: CallbacksRegistry) -> Self {
        Computer {
            program,
            program_counter: 0,
            condition: Condition::default(),
            callbacks,
            registers: [0u16; 8],
        }
    }

    pub fn next_instruction(&mut self) {
        let instruction = self.load_instruction();
        log(&format!("instruction: {:?}", instruction));

        self.execute(instruction);

        self.callbacks.call_hello();

        self.program_counter += 1;
    }

    #[allow(unused_variables)]
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddInstruction(add_instruction) => {
                self.perform_add_instruction(add_instruction);
                log("add instruction done");
                log(&format!("registers: {:#?}", self.registers));
            }
            Instruction::AndInstruction(and_instruction) => todo!(),
            Instruction::Br(condition, pcoffset9) => todo!(),
            Instruction::Jmp(register) => todo!(),
            Instruction::Jsr(pcoffset11) => todo!(),
            Instruction::Jsrr(register) => todo!(),
            Instruction::Ldb(register, register1, pcoffset6) => todo!(),
            Instruction::Ldi(register, register1, pcoffset6) => todo!(),
            Instruction::Ldr(register, register1, pcoffset6) => todo!(),
            Instruction::Lea(register, pcoffset9) => todo!(),
            Instruction::Not(register, register1) => todo!(),
            Instruction::Ret => todo!(),
            Instruction::Rti => todo!(),
            Instruction::Shf(register, register1, bit, bit1, immediate4) => todo!(),
            Instruction::Stb(register, register1, pcoffset6) => todo!(),
            Instruction::Sti(register, register1, pcoffset6) => todo!(),
            Instruction::Str(register, register1, pcoffset6) => todo!(),
            Instruction::Trap(trap_vect8) => todo!(),
        }
    }

    pub fn load_register(&self, register: Register) -> u16 {
        let index = register.to_index();
        self.registers[index]
    }

    pub fn store_register(&mut self, register: Register, value: u16) {
        let index = register.to_index();
        self.registers[index] = value;
    }

    pub fn load_instruction(&self) -> Instruction {
        self.program.instructions[self.program_counter as usize]
    }

    pub fn perform_add_instruction(&mut self, add_instruction: AddInstruction) {
        match add_instruction {
            AddInstruction::AddReg(register, register1, register2) => {
                let value1 = self.load_register(register1);
                let value2 = self.load_register(register2);
                self.store_register(register, value1 + value2);
            }
            AddInstruction::AddImm(register, register1, immediate5) => {
                let value1 = self.load_register(register1);
                let value2 = immediate5.to_value();
                self.store_register(register, value1 + value2);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Condition {
    n: bool,
    z: bool,
    p: bool,
}
