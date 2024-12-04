use wasm_bindgen::prelude::*;

mod kernel;
mod hal;
mod userspace;

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}