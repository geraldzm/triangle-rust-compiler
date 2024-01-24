use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{declaration::Declaration, identifier::Identifier, type_denoter::TypeDenoter};

pub struct VarDeclaration {
    identifier: Identifier,
    denoter: Box<dyn TypeDenoter>,
    position: SourcePosition,
}

impl VarDeclaration {
    pub fn new(
        identifier: Identifier,
        denoter: Box<dyn TypeDenoter>,
        position: SourcePosition,
    ) -> Self {
        Self {
            identifier,
            denoter,
            position,
        }
    }
}

impl Declaration for VarDeclaration {
    fn visit(&self) {
        todo!()
    }
}
