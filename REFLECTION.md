1. Dot handling in numbers
The gate in src/scanner.rs:122-128 (if self.peek() == '.' && self.is_digit(self.peek_next())) requires an immediate digit after a dot to treat it as fractional. For 5., peek() is . but peek_next() is \0, so the gate fails: 5 emits as a NUMBER, and the trailing dot is rejected as an invalid token, matching section 1.4.

2. Line counter and EOF line
Line counts increment on \n in src/scanner.rs:91-92 and 109-111. Per section 6.1, EOF uses self.last_token_line (src/scanner.rs:34-38), which records self.line when the last real token is emitted (src/scanner.rs:192-197). Trailing blank lines increment the counter but create no tokens, so EOF correctly reports the line of the last actual token rather than the file's final line.

3. Failed phase-1 test & lesson learned
I failed tests/phase-1/valid/eof_line.kobo because add() in src/scanner.rs:192-197 incremented line before recording the token, drifting all token lines down by one.

Bug: self.line += 1;

Fix: self.last_token_line = self.line;

Lesson: Advance the line counter only when consuming \n, never while constructing a token.