use crate::syntactic_analyzer::source_position::SourcePosition;

pub struct IntegerLiteral {
    spelling: String,
    position: SourcePosition,
}

impl IntegerLiteral {
    pub fn new(spelling: String, position: SourcePosition) -> Self {
        Self { spelling, position }
    }
}
