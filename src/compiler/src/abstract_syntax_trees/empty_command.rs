use crate::syntactic_analyzer::source_position::SourcePosition;

use super::expression::Expression;

pub struct EmptyCommand {
    position: SourcePosition,
}

impl EmptyCommand {
    pub fn new(position: SourcePosition) -> Self {
        Self { position }
    }
}

impl Expression for EmptyCommand {
    fn visit(&self) {
        todo!()
    }
}
