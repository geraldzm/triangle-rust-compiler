use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{
    declaration::Declaration, formal_parameter::FormalParameter, identifier::Identifier,
    type_denoter::TypeDenoter,
};

pub struct VarFormalParameter {
    i_ast: Identifier,
    t_ast: Box<dyn TypeDenoter>,
    position: SourcePosition,
}

impl VarFormalParameter {
    pub fn new(i_ast: Identifier, t_ast: Box<dyn TypeDenoter>, position: SourcePosition) -> Self {
        Self {
            i_ast,
            t_ast,
            position,
        }
    }
}

impl FormalParameter for VarFormalParameter {}

impl Declaration for VarFormalParameter {
    fn visit(&self) {
        todo!()
    }
}
