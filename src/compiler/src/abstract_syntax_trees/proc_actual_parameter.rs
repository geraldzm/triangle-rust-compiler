use crate::syntactic_analyzer::source_position::SourcePosition;


use super::{ ast::AST, identifier::Identifier, type_denoter::TypeDenoter, visitor::Visitor};


pub struct ProcActualParameter<T: AST, U: TypeDenoter + AST> {
    position: SourcePosition,
    identifier: Identifier<T, U>  
}


impl<T: AST, U: TypeDenoter + AST> ProcActualParameter<T, U> {
    fn new(position: SourcePosition, i_ast: Identifier<T, U>) -> Self {
        ProcActualParameter{
            position,
            identifier: i_ast
        }
    }
}

impl<T: AST, U: TypeDenoter + AST> AST for ProcActualParameter<T, U> {
    fn get_position<'a>(&'a self) -> &'a SourcePosition {
        &self.position
    }

    fn visit<A, G>(&mut self, visitor: &impl Visitor, t: A) -> G {
        todo!()
    }

}
