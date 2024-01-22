use crate::syntactic_analyzer::source_position::SourcePosition;

pub struct CharacterLiteral {
    spelling: String,
    position: SourcePosition,
}

impl CharacterLiteral {
    pub fn new(spelling: String, position: SourcePosition) -> Self {
        Self { spelling, position }
    }
}
