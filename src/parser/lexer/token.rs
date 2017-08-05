use std::rc::Rc;

pub struct Token {
    file: Rc<Box<str>>,
    line: usize,
    column: usize,
    token_type: TokenType,
}

impl Token {
    pub fn get_file(&self) -> &str {
        &self.file
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
}

pub enum TokenType {
    /// A generic word, this is keywords like "if" and identifiers.
    Word(String),
    /// Represents a string, e.g. "Hi!"
    String(String),
    /// Represents a BYOND number, e.g. 10, #.INF, 5e+10
    Number(f32),
    /// Represents an increase in the indentation level.
    Indent,
    /// Represents a decrease in the indentation level.
    Deindent,
    Newline,
    Semicolon
}