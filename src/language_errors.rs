#[derive(Debug)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub hint: Option<String>,
}

impl CompilerError {
    pub fn new(message: String, line: usize, column: usize, hint: Option<String>) -> CompilerError {
        let err_message = &format!("at line {}", line);

        return CompilerError {
            line,
            column,
            message: format!("{} {}.", message, err_message),
            hint,
        };
    }
}