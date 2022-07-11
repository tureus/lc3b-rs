pub mod abi;

mod computer;
pub use computer::*;

mod program;
pub use program::*;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub fn new_computer() -> Computer;
}
