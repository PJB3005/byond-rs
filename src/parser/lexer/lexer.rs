use std::io::Read;
use std::rc::Rc;
use std::collections::{HashMap, };
use parser::lexer::Token;

pub fn lex(file: &str) -> Vec<Token> {
    // Stack of files we're putting on hold due to an #include.
    // Parsing starts with this empty,
    // when an include gets encountered we push the old file and line number onto the stack.
    // Then we parse the new file, once we reach EOF we pop and continue.
    let file_stack: Vec<(Rc<Box<str>>, usize)> = Vec::new();
    let mut current_file_name = Rc::new(file.to_owned().into_boxed_str());
}

