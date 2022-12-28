use super::stripe::StripedPattern;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternKind {
    StripedPattern(StripedPattern),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternKind,
}

impl Pattern {
    pub fn new(pattern: PatternKind) -> Pattern {
        Pattern { pattern }
    }
}
