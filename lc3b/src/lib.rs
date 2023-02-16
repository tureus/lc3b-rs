mod computer;

mod error;
pub use error::*;

use bitvec::vec::BitVec;
pub use computer::*;

mod program;
pub use program::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_program(program: &str) {
    let program = Program::from_assembly(program).unwrap();
    log(&format!("{:#?}", program));
}

#[wasm_bindgen]
pub fn new_computer(program: &str) -> Computer {
    let program = Program::from_assembly(program).unwrap();
    log(&format!("{:#?}", program));
    Computer::new(program)
}
