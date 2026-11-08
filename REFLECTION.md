CA2 Reflection

1. Dangling else
In src/parser.rs:168-172, statement() parses the then-branch before the parser looks for an else. In nested ifs, this call parses the inner statement—and its optional else—completely before returning to the outer if_statement, naturally binding the else to the nearest if.
For if (false) if (true) print "a"; else print "b";, it prints (if false (if true (print "a") (print "b"))) instead of attaching the else to the outer block.
Before (2a57af8): todo!("if_statement")
Fix (2a85c30): Added let else_branch = if self.matches(&[TokenType::Else]) {


2. Assignment
In src/parser.rs:223-237, left is parsed as a regular expression first. Only when an = is encountered does match left verify it was a bare Expr::Variable (src/parser.rs:227). The parser can't know if a name starts an assignment until the left expression finishes. Right-side recursion makes a = b = 2 right-associative; invalid targets like 1 = 2; emit Cannot assign to this expression.
Before (725f2fc): Stubbed to self.or()
Fix (97111b7): Replaced with let left = self.or()?; and the target validity check


3. Error recovery
src/parser.rs:429-450 resets the nesting counter and synchronizes at statement boundaries: it consumes ; or stops before declaration/statement starters (fun, var, if, while, print, return, {). This recovers parser state without dropping the next valid construct. Without the synchronise() call at src/parser.rs:18, invalid cases like tests/phase-2/invalid/expect_expression.kobo loop indefinitely on * and time out instead of emitting the expected single error.
Before (e9284d9): Blindly advanced via self.current += 1;
Fix (dfd1e1d): Added explicit checks for ; and statement-starting tokens