use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const SYNTHETIC: Span = Span { start: 0, end: 0 };

    pub fn new(start: usize, end: usize) -> Self { Self { start, end } }

    pub fn get_length(&self) -> usize { self.end - self.start }

    pub fn new_by_spans(span_start: Self, span_end: Self) -> Self { Self::new(span_start.start, span_end.end) }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}