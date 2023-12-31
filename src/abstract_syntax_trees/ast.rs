use crate::syntactic_analyzer::source_position::SourcePosition;

pub struct AST {
    _source_position: SourcePosition
}

impl AST {
    fn _new(source_position: SourcePosition) -> AST {
        AST { _source_position: source_position }
    }
}


pub trait Visitable {
    fn visit<T, G>(&mut self, t: T) -> G;
}