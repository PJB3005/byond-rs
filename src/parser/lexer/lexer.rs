use std::borrow::ToOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{PathBuf, Path};
use std::rc::Rc;
use parser::lexer::{Token, PreprocessorItem};
use parser::lexer::charstream::CharStream;

pub fn lex<P: AsRef<Path>>(file: P) -> HashMap<PathBuf, Vec<Token>> {
    let mut lexer = Lexer::new();

    lexer.lex(file).unwrap()
}

#[derive(Debug)]
pub enum LexError {
    /// Code failed to parse.
    Parse(String),

    /// IO Error
    IO(io::Error),
}

impl From<io::Error> for LexError {
    fn from(error: io::Error) -> LexError {
        LexError::IO(error)
    }
}


struct Lexer {
    preprocessor: HashMap<String, PreprocessorItem>,
}


impl Lexer {
    pub fn new() -> Lexer {
        Lexer { preprocessor: HashMap::new() }
    }

    pub fn lex<P: AsRef<Path>>(&mut self, file: P) -> Result<Vec<Token>, LexError> {
        let mut output = Vec::new();

        let file_contents = {
            let mut file = File::open(&file)?;
            let mut string = String::new();
            file.read_to_string(&mut string)?;
            CharStream::new(&string)
        };

        let mut context = LexContext {
            file: Rc::new(file.as_ref().to_owned()),
            stream: file_contents,
            line: 0,
            indentation: 0,
        };

        // TODO: Yes, this needs to be in a loop of some sort, probably over lines.
        // I'm writing this stuff out first so I have an idea what I need first.

        // Get indentation for this line.
        let indentation = self.get_indentation(&mut context);

        // See what the next things are.
        // If it's preprocessor, a comment, EOL or EOL we don't care about indents.
        match context.stream.next() {
            Some('#') => {
                // Preprocessor.
                self.read_preprocessor_statement(&mut context)?;
            }
            Some('/') => {
                // MIGHT be comment.
                match context.stream.peek() {
                    Some('/') => self.read_single_comment()?,
                    Some('*') => self.read_multi_comment()?,
                    // Could be a type declaration, wait it out.
                    Some(c) => {}
                    None => return Err(LexError::Parse("Expected /, * or word, found EOF.")),
                }
            }
            Some('\n') => {
                // Newline.
            }

            None => return Ok(output), // Huh EOF that's convenient.
        }

        // Done, file's empty.
        Ok(output)
    }

    /// Returns the amount of indentation (tabs) from the current context.
    /// If the return value is `None`, the indentation is broken (contains spaces)
    /// Note that it ALWAYS advances to the next non-whitespace character.
    fn get_indentation(&mut self, context: &mut LexContext) -> Option<usize> {
        let (_, chars) = context.stream.read_until(|c| c == ' ' || c == '\t');
        if chars.contains(&' ') {
            // Spaces, so broken indent, but still whitespace.
            None
        } else {
            // Can only be tabs so length is the amount of characters.
            Some(chars.len())
        }
    }

    fn read_single_comment(&mut self, context: &mut LexContext) -> Result<()> {}
}

struct LexContext {
    pub file: PathBuf,
    pub stream: CharStream,
    pub line: usize,
    pub indentation: usize,
}