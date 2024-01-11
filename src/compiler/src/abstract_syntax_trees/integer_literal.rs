use crate::syntactic_analyzer::source_position::SourcePosition;

use super::{terminal::Terminal, ast::AST, visitor};

pub struct IntegerLiteral {
    spelling: String,
    position: SourcePosition
}

    // public IntegerLiteral (String theSpelling, SourcePosition thePosition) {
    //     super (theSpelling, thePosition);
    //   }
    
    //   public Object visit(Visitor v, Object o) {
    //     return v.visitIntegerLiteral(this, o);
    //   }

impl Terminal for IntegerLiteral {
    fn new(spelling: String, position: SourcePosition) -> Self {
        IntegerLiteral {
            spelling,
            position
        }
    }
}

impl AST for IntegerLiteral {
    fn get_position<'a>(&'a self) -> &'a SourcePosition {
        todo!()
    }

    fn visit<T, G>(&mut self, visitor: &impl visitor::Visitor, t: T) -> G {
        todo!()
    }
}