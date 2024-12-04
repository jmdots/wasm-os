use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Display {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Display {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Display, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width();
        let height = canvas.height();

        Ok(Self {
            context,
            width,
            height,
        })
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    pub fn write_text(&self, text: &str, x: f64, y: f64) {
        self.context.set_font("16px monospace");
        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.fill_text(text, x, y).unwrap();
    }

    pub fn draw_cursor(&self, x: f64, y: f64) {
        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.fill_rect(x, y, 8.0, 2.0);
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> u32 {
        self.height
    }
}