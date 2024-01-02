use std::fmt::Display;

pub struct SourcePosition {
    start: usize,
    finish: usize,
}

impl SourcePosition {
    pub fn new() -> SourcePosition {
        SourcePosition {
            start: 0,
            finish: 0,
        }
    }

    pub fn new_with(start: usize, finish: usize) -> SourcePosition {
        SourcePosition { start, finish }
    }
}

impl Display for SourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start, self.finish)
    }
}
