use crate::syntactic_analyzer::source_position::SourcePosition;

use super::declaration::Declaration;

pub struct SequentialDeclaration {
    d1: Box<dyn Declaration>,
    d2: Box<dyn Declaration>,
    pos: SourcePosition,
}

impl SequentialDeclaration {
    pub fn new(d1: Box<dyn Declaration>, d2: Box<dyn Declaration>, pos: SourcePosition) -> Self {
        Self { d1, d2, pos }
    }
}
impl Declaration for SequentialDeclaration {
    fn visit(&self) {
        todo!()
    }
}
