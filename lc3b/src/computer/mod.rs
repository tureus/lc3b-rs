use wasm_bindgen::prelude::*;

use crate::wasm::Callback;
use crate::Program;

pub struct CallbacksRegistry {
    pub hello: Callback,
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Computer {
    program: Program,
    program_counter: u16,
    condition: Condition,
    callbacks: CallbacksRegistry,
}

impl Computer {
    pub fn new(program: Program, callbacks: CallbacksRegistry) -> Self {
        Computer {
            program,
            program_counter: 0,
            condition: Condition::default(),
            callbacks,
        }
    }

    pub fn step<F: Fn()>(_callback: F) {}
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Condition {
    n: bool,
    z: bool,
    p: bool,
}
