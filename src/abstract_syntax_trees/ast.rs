use crate::syntactic_analyzer::source_position::SourcePosition;

use super::visitor::Visitor;

pub trait AST {
    fn get_position<'a>(&'a self) -> &'a SourcePosition;
    fn visit<T, G>(&mut self, visitor: impl Visitor, t: T) -> G;
}
