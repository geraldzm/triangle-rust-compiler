use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{
    declaration::Declaration, expression::Expression,
    formal_parameter_sequence::FormalParameterSequence, identifier::Identifier,
    type_denoter::TypeDenoter,
};

pub struct FuncDeclaration {
    i_ast: Identifier,
    fps_ast: FormalParameterSequence,
    t_denoter: Box<dyn TypeDenoter>,
    e_ast: Box<dyn Expression>,
    pos: SourcePosition,
}

impl FuncDeclaration {
    pub fn new(
        i_ast: Identifier,
        fps_ast: FormalParameterSequence,
        t_denoter: Box<dyn TypeDenoter>,
        e_ast: Box<dyn Expression>,
        pos: SourcePosition,
    ) -> Self {
        Self {
            i_ast,
            fps_ast,
            t_denoter,
            e_ast,
            pos,
        }
    }
}
impl Declaration for FuncDeclaration {
    fn visit(&self) {
        todo!()
    }
}
