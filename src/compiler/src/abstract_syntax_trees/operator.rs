use crate::syntactic_analyzer::source_position::SourcePosition;

pub struct Operator {
    spelling: String,
    position: SourcePosition,
}

impl Operator {
    pub fn new(spelling: String, position: SourcePosition) -> Self {
        Self { spelling, position }
    }
}
