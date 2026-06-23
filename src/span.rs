#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self { Self { start, end } }

    pub fn get_length(&self) -> usize { self.end - self.start }

    pub fn new_by_spans(span_start: Self, span_end: Self) -> Self { Self::new(span_start.start, span_end.end) }
}