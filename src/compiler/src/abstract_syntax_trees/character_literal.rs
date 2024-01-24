use crate::syntactic_analyzer::source_position::SourcePosition;

use super::expression::Expression;

pub struct CharacterLiteral {
    spelling: String,
    position: SourcePosition,
}

impl CharacterLiteral {
    pub fn new(spelling: String, position: SourcePosition) -> Self {
        Self { spelling, position }
    }
}

impl Expression for CharacterLiteral {
    fn visit(&self) {
        todo!()
    }
}
