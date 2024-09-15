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
    pub fn new(program: Program, js_callbacks: js_sys::Function) -> Self {
        Computer {
            program,
            program_counter: 0,
            condition: Condition::default(),
        }
    }

    pub fn step<F: Fn() -> ()>(_callback: F) {}
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Condition {
    n: bool,
    z: bool,
    p: bool,
}
