use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    path: String,
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(path: String, line: usize, column: usize) -> Self {
        Self {
            path,
            line,
            column,
        }
    }

    pub fn advance(&mut self, c: &char) {
        if *c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "<{:<40}> - ({:>7}:{:>4})",
            self.path, self.line, self.column
        )
    }
}
