#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub line: Option<usize>,
    pub column: usize,
    pub hint: Option<String>,
}

impl Error {
    pub fn new(msg: String, line: Option<usize>, column: usize, hint: Option<String>) -> Error {
        return Error {
            line,
            column,
            msg,
            hint,
        };
    }
}