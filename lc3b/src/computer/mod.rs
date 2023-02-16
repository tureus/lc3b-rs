use wasm_bindgen::prelude::*;

use crate::Program;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Computer {
    program: Program,
    program_counter: u16,
    condition: Condition,
}

impl Computer {
    pub fn new(program: Program) -> Self {
        Computer {
            program,
            program_counter: 0,
            condition: Condition::default(),
        }
    }

    pub fn step<F: Fn() -> ()>(callback: F) {}
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Condition {
    n: bool,
    z: bool,
    p: bool,
}
