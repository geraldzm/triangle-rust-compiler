use crate::syntactic_analyzer::source_position::SourcePosition;

pub trait Terminal  {
    fn new(spelling: String, position: SourcePosition) -> Self;
}
