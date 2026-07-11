//! The line folding writer required by RFC 6350 section 3.2.

use std::fmt::{self, Write};

/// A physical line must not be longer than 75 octets, not counting the line break.
const MAX_LINE_OCTETS: usize = 75;

/// A writer that folds long content lines by inserting a CRLF followed by a single space.
///
/// Folding always happens at character boundaries, so multi-octet UTF-8 characters are never split.
pub struct FoldingWriter<'a> {
    out:         &'a mut (dyn Write + 'a),
    line_octets: usize,
}

impl<'a> FoldingWriter<'a> {
    /// Wraps a plain writer so that everything written through it gets folded.
    #[inline]
    pub(crate) fn new(out: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            out,
            line_octets: 0,
        }
    }

    /// Ends the current content line by writing a CRLF.
    #[inline]
    pub(crate) fn end_line(&mut self) -> fmt::Result {
        self.line_octets = 0;

        self.out.write_str("\r\n")
    }
}

impl Write for FoldingWriter<'_> {
    fn write_str(&mut self, mut s: &str) -> fmt::Result {
        while !s.is_empty() {
            let remaining = MAX_LINE_OCTETS - self.line_octets;

            // Find the biggest prefix that fits into this line without splitting a character.
            let mut length = s.len().min(remaining);

            while !s.is_char_boundary(length) {
                length -= 1;
            }

            if length == 0 {
                // Nothing more fits, so fold here and count the leading space of the new line.
                self.out.write_str("\r\n ")?;
                self.line_octets = 1;

                continue;
            }

            self.out.write_str(&s[..length])?;
            self.line_octets += length;

            s = &s[length..];
        }

        Ok(())
    }
}
