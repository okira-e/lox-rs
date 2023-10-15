#[derive(Debug)]
pub struct CompilerError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
    pub hint: Option<String>,
}

impl CompilerError {
    pub fn new(msg: String, line: usize, column: usize, hint: Option<String>) -> CompilerError {
        return CompilerError {
            line,
            column,
            msg,
            hint,
        };
    }
}