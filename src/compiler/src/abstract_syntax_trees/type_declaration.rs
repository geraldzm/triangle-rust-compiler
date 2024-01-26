use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{declaration::Declaration, identifier::Identifier, type_denoter::TypeDenoter};

pub struct TypeDeclaration {
    identifier: Identifier,
    type_denoter: Box<dyn TypeDenoter>,
    position: SourcePosition,
}

impl TypeDeclaration {
    pub fn new(
        identifier: Identifier,
        type_denoter: Box<dyn TypeDenoter>,
        position: SourcePosition,
    ) -> Self {
        Self {
            identifier,
            type_denoter,
            position,
        }
    }
}

impl Declaration for TypeDeclaration {
    fn visit(&self) {
        todo!()
    }
}
