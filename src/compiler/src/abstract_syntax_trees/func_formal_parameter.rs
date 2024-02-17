use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{
    declaration::Declaration, formal_parameter::FormalParameter,
    formal_parameter_sequence::FormalParameterSequence, identifier::Identifier,
    type_denoter::TypeDenoter,
};

pub struct FuncFormalParameter {
    i_ast: Identifier,
    t_ast: Box<dyn FormalParameterSequence>,
    t_denoter: Box<dyn TypeDenoter>,
    position: SourcePosition,
}

impl FuncFormalParameter {
    pub fn new(
        i_ast: Identifier,
        t_ast: Box<dyn FormalParameterSequence>,
        t_denoter: Box<dyn TypeDenoter>,
        position: SourcePosition,
    ) -> Self {
        Self {
            i_ast,
            t_ast,
            t_denoter,
            position,
        }
    }
}

impl Declaration for FuncFormalParameter {
    fn visit(&self) {
        todo!()
    }
}

impl FormalParameter for FuncFormalParameter {}
