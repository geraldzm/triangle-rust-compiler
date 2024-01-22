use crate::syntactic_analyzer::source_position::SourcePosition;

use super::character_literal::CharacterLiteral;

pub struct CharacterExpression {
    char_literal: CharacterLiteral,
    position: SourcePosition,
}

impl CharacterExpression {
    pub fn new(char_literal: CharacterLiteral, position: SourcePosition) -> Self {
        Self {
            char_literal,
            position,
        }
    }
}
