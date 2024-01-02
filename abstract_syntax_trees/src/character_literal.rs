use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{ast::AST, visitor::Visitor};



pub struct CharacterLiteral {
    spelling: String,
    position: SourcePosition   
}

impl CharacterLiteral {
    fn new(spelling: String, position: SourcePosition) -> Self {
        CharacterLiteral {
            spelling,
            position
        }
    }
}


impl AST for CharacterLiteral {
    fn get_position<'a>(&'a self) -> &'a SourcePosition {
        &self.position
    }

    fn visit<T, G>(&mut self, visitor: &impl Visitor, t: T) -> G {
        todo!()
    }

}