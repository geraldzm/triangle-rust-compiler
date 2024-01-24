use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{declaration::Declaration, expression::Expression, identifier::Identifier};

pub struct ConstDeclaration {
    identifier: Identifier,
    expression: Box<dyn Expression>,
    position: SourcePosition,
}

impl ConstDeclaration {
    pub fn new(
        identifier: Identifier,
        expression: Box<dyn Expression>,
        position: SourcePosition,
    ) -> Self {
        Self {
            identifier,
            expression,
            position,
        }
    }
}

impl Declaration for ConstDeclaration {
    fn visit(&self) {
        todo!()
    }
}
