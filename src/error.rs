use std::fmt;

/// A parse failure tied to an exact position in the source text.
///
/// `line` and `column` are both 1-based, matching how editors and compilers
/// report positions, so a user can jump straight to the offending character.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl ParseError {
    pub fn new(line: usize, column: usize, message: impl Into<String>) -> Self {
        ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Render the error together with the offending source line and a caret
    /// under the column, the way a compiler diagnostic would.
    pub fn render(&self, source: &str) -> String {
        let line_text = source.lines().nth(self.line.saturating_sub(1)).unwrap_or("");
        let gutter = self.line.to_string();
        let pad = " ".repeat(gutter.len());
        let caret_pad = " ".repeat(self.column.saturating_sub(1));
        format!(
            "error: {msg}\n{pad} |\n{gutter} | {text}\n{pad} | {caret_pad}^\n{pad} at line {line}, column {col}",
            msg = self.message,
            pad = pad,
            gutter = gutter,
            text = line_text,
            caret_pad = caret_pad,
            line = self.line,
            col = self.column,
        )
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_points_a_caret_at_the_column() {
        let e = ParseError::new(1, 5, "USD amounts use 2 fractional digit(s), but this amount has 1");
        let rendered = e.render("$12.5\n");
        let expected = "error: USD amounts use 2 fractional digit(s), but this amount has 1\n"
            .to_string()
            + " |\n"
            + "1 | $12.5\n"
            + " |     ^\n"
            + " at line 1, column 5";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_picks_the_right_line_out_of_several() {
        let e = ParseError::new(2, 3, "boom");
        let rendered = e.render("first\nsecond\nthird");
        assert!(rendered.contains("2 | second"));
        assert!(rendered.contains("  ^"));
    }

    #[test]
    fn display_impl_is_line_column_message() {
        let e = ParseError::new(4, 9, "bad");
        assert_eq!(e.to_string(), "line 4, column 9: bad");
    }
}
