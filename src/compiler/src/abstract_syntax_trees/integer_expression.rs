use crate::syntactic_analyzer::source_position::SourcePosition;

use super::integer_literal::IntegerLiteral;

pub struct IntegerExpression {
    int_literal: IntegerLiteral,
    position: SourcePosition,
}

impl IntegerExpression {
    pub fn new(int_literal: IntegerLiteral, position: SourcePosition) -> Self {
        Self {
            int_literal,
            position,
        }
    }
}
