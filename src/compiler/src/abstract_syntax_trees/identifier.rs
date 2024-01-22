use crate::syntactic_analyzer::source_position::SourcePosition;

pub struct Identifier {
    spelling: String,
    position: SourcePosition,
}

impl Identifier {
    pub fn new(spelling: String, position: SourcePosition) -> Self {
        Self { spelling, position }
    }
}
