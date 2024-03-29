// ConstFormalParameter

use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{
    declaration::Declaration, formal_parameter::FormalParameter, identifier::Identifier,
    type_denoter::TypeDenoter,
};

pub struct ConstFormalParameter {
    i_ast: Identifier,
    t_ast: Box<dyn TypeDenoter>,
    position: SourcePosition,
}

impl ConstFormalParameter {
    pub fn new(i_ast: Identifier, t_ast: Box<dyn TypeDenoter>, position: SourcePosition) -> Self {
        Self {
            i_ast,
            t_ast,
            position,
        }
    }
}

impl Declaration for ConstFormalParameter {
    fn visit(&self) {
        todo!()
    }
}

impl FormalParameter for ConstFormalParameter {}
