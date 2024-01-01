use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{ast::AST, terminal::Terminal, type_denoter::TypeDenoter};


pub struct Identifier<T: AST, U: TypeDenoter> {
    spelling: String, 
    position: SourcePosition,
    pub typ: Option<U>,
    pub decl: Option<T> // Either a Declaration or a FieldTypeDenoter
}

impl<T: AST, U: TypeDenoter> Terminal for Identifier<T, U> {
    fn new(spelling: String, position: SourcePosition) -> Self {
        Identifier {
            spelling,
            position,
            typ: None,
            decl: None
        }
    }
}

impl<T: AST, U: TypeDenoter> AST for Identifier<T, U> {
    fn get_position<'a>(&'a self) -> &'a SourcePosition {
        &self.position
    }

    fn visit<I, G>(&mut self, visitor: impl super::visitor::Visitor, t: I) -> G {
        todo!()
    }
}

//     public Object visit(Visitor v, Object o) {
//       return v.visitIdentifier(this, o);
//     }