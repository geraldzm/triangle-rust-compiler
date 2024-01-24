use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{expression::Expression, integer_literal::IntegerLiteral};

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

impl Expression for IntegerExpression {
    fn visit(&self) {
        todo!()
    }
}
