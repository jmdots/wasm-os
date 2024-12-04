use wasm_bindgen::prelude::*;

mod kernel;
mod hal;
mod userspace;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "panic-hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn create_shell(canvas_id: &str) -> Result<userspace::shell::Shell, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let display = hal::display::Display::new(canvas)?;
    Ok(userspace::shell::Shell::new(display))
}