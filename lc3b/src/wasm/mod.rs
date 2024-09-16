use crate::{CallbacksRegistry, Computer, Program};
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

pub enum Callback {
    JS(js_sys::Function),
}

#[wasm_bindgen]
pub struct WasmCallbacksRegistry {
    hello: js_sys::Function,
}

#[wasm_bindgen]
pub fn new_computer(program: &str, callbacks: WasmCallbacksRegistry) -> Computer {
    let program = Program::from_assembly(program).unwrap();
    log(&format!("{:#?}", program));
    if let Err(e) = callbacks.hello.call0(&JsValue::NULL) { log(&format!("failed to call hello: {:?}", e)) };

    let callbacks = CallbacksRegistry {
        hello: Callback::JS(callbacks.hello),
    };
    Computer::new(program, callbacks)
}
