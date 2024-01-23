use crate::syntactic_analyzer::source_position::SourcePosition;

use super::expression::Expression;

pub struct IfExpression {
    e1: Box<dyn Expression>,
    e2: Box<dyn Expression>,
    e3: Box<dyn Expression>,
    position: SourcePosition,
}

impl IfExpression {
    pub fn new(
        e1: Box<dyn Expression>,
        e2: Box<dyn Expression>,
        e3: Box<dyn Expression>,
        position: SourcePosition,
    ) -> Self {
        Self {
            e1,
            e2,
            e3,
            position,
        }
    }
}

impl Expression for IfExpression {
    fn visit(&self) {
        todo!()
    }
}
