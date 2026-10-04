# Reflection

## 1. Dot handling in numbers

The decision is in src/scanner.rs:122-128: `if self.peek() == '.' && self.is_digit(self.peek_next())` is the exact gate. That means a dot only starts the fractional part when another digit follows it immediately; otherwise the dot is not part of a valid literal. For `5.`, the scanner reads the `5`, then checks the following character. Since `peek()` is `.` but `peek_next()` is `\0`, the condition fails, so the `NUMBER` token is emitted and the stray `.` is then rejected as `Character is not part of any token.`. That is exactly what section 1.4 requires: a number is digits, optionally followed by a dot and one or more digits, and a lonely dot is not a token at all.

## 2. Line counter and EOF line

The line counter changes in exactly two places: src/scanner.rs:91-92 (`'\n' => self.line += 1`) and src/scanner.rs:109-111 (`if c == '\n' { self.line += 1; }`). Every other token relies on the current `line` value, and the final `EOF` is created in src/scanner.rs:34-38 with `line: self.last_token_line`. The last real token sets that value in src/scanner.rs:192-197 via `self.last_token_line = self.line;`. For a file that ends in two blank lines, the scanner has advanced past the last real token and the extra newline characters, but it never created new tokens for those blank lines, so the `EOF` token still carries the line of the last real token, not the file's final line. Section 6.1 asks for that because editor-created trailing newlines are not real source lines for token reporting.

## 3. A failed phase-1 test and the lesson learned

I failed tests/phase-1/valid/eof_line.kobo. The fix was in src/scanner.rs:192-197, in `add()`. I had misunderstood the order of operations: I was incrementing `line` before adding the token, so the token was reported on the next line. In other words, the counter was being advanced in the wrong place, which made `PRINT`, `NUMBER`, `SEMICOLON`, and `EOF` all drift one line downward. The wrong commit was `0a36ced`, and the line as it stood there was:

```rust
self.line += 1;
```

The fix commit was `a3d7e01`, and the corrected line was:

```rust
self.last_token_line = self.line;
```

The real lesson is that newline counting belongs in the scanner when it consumes a newline, not in the token-construction step that records the line for a token that has already been read.
