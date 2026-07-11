//! The iterator that unfolds physical lines into logical content lines.

use std::borrow::Cow;

/// Iterates over unfolded logical lines together with their first physical line numbers.
///
/// Both CRLF and bare LF line breaks are accepted, and blank lines are skipped.
pub(crate) struct LogicalLines<'a> {
    input:       &'a str,
    position:    usize,
    line_number: usize,
}

impl<'a> LogicalLines<'a> {
    #[inline]
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line_number: 1,
        }
    }

    /// Returns the number of the physical line that would be read next.
    #[inline]
    pub(crate) fn current_line_number(&self) -> usize {
        self.line_number
    }

    /// Takes the next physical line, removing the line break.
    fn take_physical_line(&mut self) -> &'a str {
        let rest = &self.input[self.position..];

        let (line, consumed) = match rest.find('\n') {
            Some(i) => (&rest[..i], i + 1),
            None => (rest, rest.len()),
        };

        self.position += consumed;
        self.line_number += 1;

        line.strip_suffix('\r').unwrap_or(line)
    }

    /// Checks whether the next physical line is a folded continuation, which starts with a space or a tab.
    #[inline]
    fn at_continuation(&self) -> bool {
        self.input[self.position..].starts_with([' ', '\t'])
    }
}

impl<'a> Iterator for LogicalLines<'a> {
    type Item = (usize, Cow<'a, str>);

    fn next(&mut self) -> Option<Self::Item> {
        // Find the first physical line that is not blank.
        let (number, line) = loop {
            if self.position >= self.input.len() {
                return None;
            }

            let number = self.line_number;

            let line = self.take_physical_line();

            if !line.is_empty() {
                break (number, line);
            }
        };

        // The common case has no folding, so the line can be borrowed without copying.
        if !self.at_continuation() {
            return Some((number, Cow::Borrowed(line)));
        }

        let mut unfolded = String::with_capacity(line.len() * 2);

        unfolded.push_str(line);

        while self.at_continuation() {
            // Unfolding removes the line break and exactly one leading space or tab.
            unfolded.push_str(&self.take_physical_line()[1..]);
        }

        Some((number, Cow::Owned(unfolded)))
    }
}
