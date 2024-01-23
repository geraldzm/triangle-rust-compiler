use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{expression::Expression, operator::Operator};

pub struct BinaryExpression {
    e1: Box<dyn Expression>,
    operator: Operator,
    e2: Box<dyn Expression>,
    position: SourcePosition,
}

impl BinaryExpression {
    pub fn new(
        e1: Box<dyn Expression>,
        operator: Operator,
        e2: Box<dyn Expression>,
        position: SourcePosition,
    ) -> Self {
        Self {
            e1,
            operator,
            e2,
            position,
        }
    }
}

impl Expression for BinaryExpression {
    fn visit(&self) {
        todo!()
    }
}
