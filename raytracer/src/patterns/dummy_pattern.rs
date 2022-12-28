use super::pattern::{Pattern, PatternKind};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn pattern() -> Pattern {
        Pattern::new(PatternKind::TestPattern(TestPattern {}))
    }
}
