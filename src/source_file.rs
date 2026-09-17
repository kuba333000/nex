#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
    line_starts: Vec<usize>,
    id: usize,
}

impl SourceFile {
    pub fn new(id: usize, path: String, content: String) -> Self { Self { id, path, content, line_starts: Vec::new() } }

    pub fn fill_line_starts(&mut self) {
        self.line_starts = vec![0];

        self.line_starts.extend(
            self.content.bytes()
                .enumerate()
                .filter_map(|(i, b)| (b == b'\n').then_some(i + 1)),
        );
    }

    pub fn get_lines(&mut self, start_line_num: usize, end_line_num: usize) -> Vec<&str> {
        self.fill_line_starts();

        let mut lines = Vec::new();

        for line_num in start_line_num..=end_line_num {
            if line_num >= self.line_starts.len() { break; };

            let start = self.line_starts[line_num];
            let end = if line_num + 1 < self.line_starts.len() {
                self.line_starts[line_num + 1]
            } else {
                self.content.len()
            };

            lines.push(self.content[start..end].trim_end_matches('\n'));
        }

        lines
    }

    pub fn pos_from_char(&mut self, offset: usize) -> (usize, usize) {
        self.fill_line_starts();

        let line = match self.line_starts.binary_search(&offset) {
            Ok(line) => line,
            Err(line) => line - 1,
        };

        let column = offset - self.line_starts[line];

        (line, column)
    }
}