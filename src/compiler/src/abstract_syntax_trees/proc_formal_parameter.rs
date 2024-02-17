// ConstFormalParameter

use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{
    declaration::Declaration, formal_parameter::FormalParameter,
    formal_parameter_sequence::FormalParameterSequence, identifier::Identifier,
};

pub struct ProcFormalParameter {
    i_ast: Identifier,
    t_ast: Box<dyn FormalParameterSequence>,
    position: SourcePosition,
}

impl ProcFormalParameter {
    pub fn new(
        i_ast: Identifier,
        t_ast: Box<dyn FormalParameterSequence>,
        position: SourcePosition,
    ) -> Self {
        Self {
            i_ast,
            t_ast,
            position,
        }
    }
}

impl Declaration for ProcFormalParameter {
    fn visit(&self) {
        todo!()
    }
}

impl FormalParameter for ProcFormalParameter {}
