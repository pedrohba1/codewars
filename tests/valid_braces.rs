fn valid_braces(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if Bracket::closing(c) {
            if let Some(opening) = stack.last().copied() {
                if Bracket::is_matching_pair(Some(opening), Some(c)) {
                    stack.pop();
                } else {
                    return false;
                }
                continue;
            }
        }
        stack.push(c);
    }
    stack.is_empty()
}

#[derive(Debug)]
struct Bracket;

impl Bracket {
    fn is_matching_pair(open: Option<char>, close: Option<char>) -> bool {
        matches!(
            (open, close),
            (Some('{'), Some('}')) | (Some('['), Some(']')) | (Some('('), Some(')'))
        )
    }

    fn opening(c: char) -> bool {
        matches!(c, '{' | '[' | '(')
    }

    fn closing(c: char) -> bool {
        matches!(c, '}' | ']' | ')')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
        expect_true("{([])}");
        expect_false("[{[(");
    }

    fn expect_true(s: &str) {
        assert!(
            valid_braces(s),
            "Expected {s:?} to be valid. Got false",
            s = s
        );
    }

    fn expect_false(s: &str) {
        assert!(
            !valid_braces(s),
            "Expected {s:?} to be invalid. Got true",
            s = s
        );
    }
}
