use crate::{Computer, Program};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_program(program: &str) {
    let program = Program::from_assembly(program);
    match program {
        Ok(p) => log(&format!("{:#?}", p)),
        Err(e) => log(&format!("error: {:?}", e)),
    }
}

#[wasm_bindgen]
pub fn new_computer(program: &str, hello: js_sys::Function) -> Computer {
    let program = Program::from_assembly(program).unwrap();
    log(&format!("{:#?}", program));
    match hello.call0(&JsValue::NULL) {
        Err(e) => log(&format!("failed to call hello: {:?}", e)),
        _ => (),
    };
    Computer::new(program, hello)
}
