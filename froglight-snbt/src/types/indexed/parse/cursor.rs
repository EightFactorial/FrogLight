#![expect(unused, reason = "WIP")]

use core::range::Range;

#[derive(Debug, Clone)]
pub(super) struct Cursor<'data> {
    root: &'data str,
    position: usize,
}

impl<'data> Cursor<'data> {
    /// Create a new [`Cursor`].
    #[inline]
    #[must_use]
    pub(super) const fn new(root: &'data str) -> Self { Self { root, position: 0 } }

    /// Get the current position of the cursor.
    #[inline]
    #[must_use]
    pub(super) const fn position(&self) -> usize { self.position }

    /// Advance the cursor by `count` bytes.
    ///
    /// Checks that the cursor has enough remaining data before consuming it.
    #[inline]
    const fn consume(&mut self, count: usize) {
        debug_assert!(self.position + count <= self.root.len(), "CURSOR OUT OF BOUNDS!");
        self.position += count;
    }

    /// Advance the cursor, consuming the given string slice.
    ///
    /// Checks that the cursor starts with the slice before consuming it.
    #[inline]
    fn consume_slice(&mut self, slice: &'data str) {
        debug_assert!(self.remaining().starts_with(slice), "CURSOR DOESN'T CONTAIN SLICE");
        self.consume(slice.len());
    }

    /// Get the consumed, parsed data.
    #[inline]
    #[must_use]
    pub(super) fn consumed(&self) -> &'data str {
        // SAFETY: `position` is always in-bounds and on a char boundary.
        unsafe { self.root.get_unchecked(..self.position) }
    }

    /// Get the remaining unparsed data.
    #[inline]
    #[must_use]
    pub(super) fn remaining(&self) -> &'data str {
        // SAFETY: `position` is always in-bounds and on a char boundary.
        unsafe { self.root.get_unchecked(self.position..) }
    }

    /// Trim the start of the string, advancing past any whitespace.
    pub(super) fn trim_start(&mut self) -> &mut Self {
        let trimmed = self.remaining().trim_start();
        self.consume(self.remaining().len() - trimmed.len());
        self
    }

    /// Create a new cursor over the given range, advancing the current cursor
    /// past it.
    #[must_use]
    pub(super) fn split_range(&mut self, range: Range<usize>) -> Self {
        debug_assert_eq!(self.position, range.start, "CURSOR NOT AT RANGE START");
        let cursor = self.clone();
        self.consume(range.end - range.start);
        cursor
    }

    /// Take a slice of the given length, advancing the cursor.
    #[must_use]
    pub(super) fn take_slice(&mut self, length: usize) -> &'data str {
        let slice = unsafe { self.root.get_unchecked(self.position..self.position + length) };
        self.consume_slice(slice);
        slice
    }

    /// Peek the next character without consuming it.
    #[inline]
    #[must_use]
    pub(super) fn peek(&self) -> Option<char> { self.remaining().chars().next() }

    /// Peek the second next character without consuming it.
    #[inline]
    #[must_use]
    pub(super) fn peek2(&self) -> Option<char> {
        let mut chars = self.remaining().chars();
        chars.next()?;
        chars.next()
    }

    /// Get the next character, consuming it.
    #[inline]
    #[must_use]
    pub(super) fn next(&mut self) -> Option<char> {
        let char = self.remaining().chars().next()?;
        self.consume(char.len_utf8());
        Some(char)
    }

    /// Get the next character and check that it matches the expected character.
    #[inline]
    pub(super) fn next_expect(&mut self, expected: char) -> Result<(), ()> {
        if self.next() == Some(expected) { Ok(()) } else { Err(()) }
    }

    /// Advance the cursor until a closure return true,
    /// returning the string up to that point.
    ///
    /// # Generics
    ///
    /// `ESCAPABLE` controls whether the character can be escaped with a
    /// backslash.
    ///
    /// `INCLUSIVE` controls whether the returned string includes the target
    /// character.
    #[must_use]
    pub(super) fn until_char<
        const ESCAPABLE: bool,
        const INCLUSIVE: bool,
        P: FnMut(char) -> bool,
    >(
        &mut self,
        mut pattern: P,
    ) -> &'data str {
        let mut output = self.remaining();

        let mut last_a = '\0';
        let mut last_b = '\0';
        for (index, char) in output.char_indices() {
            if pattern(char) {
                // Calculate `index` based on `INCLUSIVE`.
                let index = if INCLUSIVE { index + char.len_utf8() } else { index };

                // If `ESCAPABLE`, break if the target isn't escaped.
                // (If `last_a` is a backslash, then `last_b` must not be a backslash)
                if ESCAPABLE && !((last_a == '\\') && (last_b != '\\')) {
                    output = self.take_slice(index);
                    break;
                }
                // If not `ESCAPABLE`, break immediately.
                if !ESCAPABLE {
                    output = self.take_slice(index);
                    break;
                }
            }

            // `last_b` <- `last_a` <- `char`
            (last_b, last_a) = (last_a, char);
        }

        output
    }
}
