use std::fmt::Display;

#[derive(Clone)]
pub struct SourcePosition {
    pub start: usize,
    pub finish: usize,
}

impl SourcePosition {
    pub fn new() -> Self {
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
