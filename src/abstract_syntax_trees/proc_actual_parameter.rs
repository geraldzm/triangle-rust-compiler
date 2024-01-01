use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{actual_parameter::ActualParameter, ast::AST, identifier::Identifier, type_denoter::TypeDenoter};



pub struct ProcActualParameter<T: AST, U: TypeDenoter + AST> {
    position: SourcePosition,
    identifier: Identifier<T, U>  
}

impl ActualParameter for ProcActualParameter<T impl AST, U: impl TypeDenoter + AST>{}

impl ProcActualParameter {
    fn new(position: SourcePosition, identifier: Identifier) -> Self {
        ProcActualParameter{
            position,
            identifier
        }
    }
}

impl AST for ProcActualParameter {
    fn get_position<'a>(&'a self) -> &'a SourcePosition {
        &self.position
    }

    fn visit<T, G>(&mut self, visitor: impl super::visitor::Visitor, t: T) -> G {
        todo!()
    }
}


//       public Object visit(Visitor v, Object o) {
//         return v.visitProcActualParameter(this, o);
//       }