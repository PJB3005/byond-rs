/// A stream of characters,
/// To allow easier peeking and managing of state.
pub struct CharStream {
    // NOT a string because it makes iteration and peeking *really* difficult.
    buffer: Vec<char>,
    // Position that will be read next time next() is called.
    position: usize,
}

impl CharStream {
    pub fn new(string: &str) -> CharStream {
        CharStream {
            // Hard-ignore CR so the input is always LF EOLs.
            buffer: string.chars().filter(|c| c != '\r').collect(),
            position: 0,
        }
    }

    /// Get the next character in the stream without advancing it.
    pub fn peek(&mut self) -> Option<char> {
        self.buffer.get(self.position).map(|c| *c)
    }

    /// Peek the character offset by `offset`.
    pub fn peek_to(&mut self, offset: isize) -> Option<char> {
        // usize + isize is hard.
        let position = if offset < 0 {
            self.position - (-offset) as usize
        } else {
            self.position + offset as usize
        };

        self.buffer.get(position).map(|c| *c)
    }

    /// Read characters and invoke `F` with them, until it returns `false`.
    /// The read pointer will be placed so that next read will return the character that failed.
    /// Return value is whether EOF got hit (bool) and the characters that were read and passed.
    pub fn read_until<F>(&mut self, mut predicate: F) -> (bool, &[char])
        where F: FnMut(char) -> bool
    {
        let starting_pos = self.position;
        let ended = loop {
            match self.next() {
                Some(c) => {
                    if !predicate(c) {
                        self.position -= 1;
                        break true;
                    }
                }
                // Hit EOF.
                _ => break false,
            }
        };

        (ended, &self.buffer[starting_pos..self.position])
    }

    pub fn skip_whitespace(&mut self) {
        self.read_until(|c| c == ' ' || c == '\t');
    }
}

// Iterator so we can use some iterator combinators on it.
impl Iterator for CharStream {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.buffer.get(self.position) {
            Some(c) => {
                self.position += 1;
                Some(*c)
            }
            None => None,
        }
    }
}