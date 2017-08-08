use std::borrow::ToOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use std::rc::Rc;
use parser::lexer::Token;

pub fn lex<P: AsRef<Path>>(file: P) -> Vec<Token> {
    // Stack of files we're putting on hold due to an #include.
    // Parsing starts with this empty,
    // when an include gets encountered we push the old file and line number onto the stack.
    // Then we parse the new file, once we reach EOF we pop and continue.
    let file_stack: Vec<(Rc<PathBuf>, usize)> = Vec::new();
    let mut current_file_name = Rc::new(file.as_ref().to_owned());

    // Main file loop, loops over files we need to read.
    loop {
        let file_contents = {
            let file = File::open(current_file_name.as_ref());
        };
    }
}
