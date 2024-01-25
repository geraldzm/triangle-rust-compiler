use crate::syntactic_analyzer::source_position::SourcePosition;

pub struct FormalParameterSequence {
    position: SourcePosition,
}

impl FormalParameterSequence {
    pub fn new(position: SourcePosition) -> Self {
        Self { position }
    }
}
