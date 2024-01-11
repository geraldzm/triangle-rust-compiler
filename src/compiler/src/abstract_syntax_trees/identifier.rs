use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{ast::AST,  type_denoter::TypeDenoter};


pub struct Identifier<T: AST, U: TypeDenoter + AST> {
    spelling: String, 
    position: SourcePosition,
    pub typ: Option<U>,
    pub decl: Option<T> // Either a Declaration or a FieldTypeDenoter
}

impl<T: AST, U: TypeDenoter + AST> Identifier<T, U> {
    fn new(spelling: String, position: SourcePosition) -> Self {
        Identifier {
            spelling,
            position,
            typ: None,
            decl: None
        }
    }
}

impl<T: AST, U: TypeDenoter + AST> AST for Identifier<T, U> {
    fn get_position<'a>(&'a self) -> &'a SourcePosition {
        &self.position
    }

    fn visit<A, G>(&mut self, visitor: &impl super::visitor::Visitor, t: A) -> G {
        todo!()
    }

}

//     public Object visit(Visitor v, Object o) {
//       return v.visitIdentifier(this, o);
//     }