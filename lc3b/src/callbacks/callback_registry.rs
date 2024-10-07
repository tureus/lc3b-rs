use wasm_bindgen::JsValue;

use crate::wasm::Callback;

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
