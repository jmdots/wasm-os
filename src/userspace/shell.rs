use wasm_bindgen::prelude::*;
use crate::hal::display::Display;

#[wasm_bindgen]
pub struct Shell {
    display: Display,
    cursor_x: f64,
    cursor_y: f64,
    input_buffer: String,
}

#[wasm_bindgen]
impl Shell {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> Self {
        let mut shell = Self {
            display,
            cursor_x: 10.0,
            cursor_y: 20.0,
            input_buffer: String::new(),
        };
        shell.clear_screen();
        shell
    }

    #[wasm_bindgen]
    pub fn handle_keypress(&mut self, key: &str) {
        match key {
            "Enter" => self.execute_command(),
            "Backspace" => {
                if !self.input_buffer.is_empty() {
                    self.input_buffer.pop();
                }
            },
            key if key.len() == 1 => {
                self.input_buffer.push_str(key);
            },
            _ => {},
        }
        self.render();
    }

    fn execute_command(&mut self) {
        let output: String = match self.input_buffer.trim() {
            "help" => {
                "Available commands:\n  help  - Show this message\n  clear - Clear the screen\n  exit  - Exit (just kidding!)\n  about - About WasmOS".to_string()
            },
            "clear" => {
                self.clear_screen();
                String::new()
            },
            "about" => {
                "WasmOS v0.1.0\nA WebAssembly-based operating system\nRunning in your browser!".to_string()
            },
            "exit" => "Note: You're running in a browser, so there's nowhere to exit to! :)".to_string(),
            cmd => format!("Unknown command: {}\nType 'help' for available commands", cmd),
        };

        if !output.is_empty() {
            for line in output.lines() {
                self.cursor_y += 20.0;
                self.display.write_text(line, 10.0, self.cursor_y);
            }
        }

        self.cursor_y += 20.0;
        self.input_buffer.clear();
    }

    fn clear_screen(&mut self) {
        self.display.clear();
        self.cursor_y = 20.0;
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        self.display.write_text("WasmOS > ", 10.0, self.cursor_y);
        self.display.write_text(&self.input_buffer, 80.0, self.cursor_y);
        self.display.draw_cursor(80.0 + self.input_buffer.len() as f64 * 9.0, self.cursor_y);
    }
}