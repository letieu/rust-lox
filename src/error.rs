#[derive(Debug)]
pub struct LoxError {
    pub message: String,
    pub line: usize,
}

impl LoxError {
    pub fn new(message: String, line: usize) -> Self {
        Self { message, line }
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.message);
    }
}
