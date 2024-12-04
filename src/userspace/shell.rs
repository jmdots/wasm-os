use wasm_bindgen::prelude::*;
use crate::hal::display::Display;

pub struct Shell {
    display: Display,
    cursor_x: f64,
    cursor_y: f64,
    input_buffer: String,
}

impl Shell {
    pub fn new(display: Display) -> Self {
        Self {
            display,
            cursor_x: 10.0,
            cursor_y: 20.0,
            input_buffer: String::new(),
        }
    }

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
            "help" => "Available commands: help, clear, exit".to_string(),
            "clear" => {
                self.display.clear();
                self.cursor_y = 20.0;
                String::new()
            },
            "exit" => "Goodbye!".to_string(),
            cmd => format!("Unknown command: {}", cmd),
        };

        if !output.is_empty() {
            self.cursor_y += 20.0;
            self.display.write_text(&output, 10.0, self.cursor_y);
        }

        self.cursor_y += 20.0;
        self.input_buffer.clear();
    }

    pub fn render(&self) {
        self.display.clear();
        self.display.write_text("WasmOS > ", 10.0, self.cursor_y);
        self.display.write_text(&self.input_buffer, 80.0, self.cursor_y);
        self.display.draw_cursor(80.0 + self.input_buffer.len() as f64 * 9.0, self.cursor_y);
    }
}