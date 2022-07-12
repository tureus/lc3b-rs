pub mod abi;

mod computer;
pub use computer::*;

mod program;
pub use program::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn new_computer(input: String) -> Computer {
    alert(&input);
    Computer::new(Program {
        instructions: vec![],
    })
}
