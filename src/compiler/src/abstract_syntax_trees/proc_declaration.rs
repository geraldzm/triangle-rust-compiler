use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{
    command::Command, declaration::Declaration, formal_parameter_sequence::FormalParameterSequence,
    identifier::Identifier,
};

pub struct ProcDeclaration {
    i_ast: Identifier,
    fps_ast: FormalParameterSequence,
    c_ast: Command,
    position: SourcePosition,
}

impl ProcDeclaration {
    pub fn new(
        i_ast: Identifier,
        fps_ast: FormalParameterSequence,
        c_ast: Command,
        position: SourcePosition,
    ) -> Self {
        Self {
            i_ast,
            fps_ast,
            c_ast,
            position,
        }
    }
}

impl Declaration for ProcDeclaration {
    fn visit(&self) {
        todo!()
    }
}
