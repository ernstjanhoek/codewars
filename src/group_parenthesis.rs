// NIET AF!

pub fn eval_parentheses(s: &str) -> u32 {
    let mut total: u32 = 0;
    let mut multiplier: u32 = 0;
    let mut score_buffer: u32 = 0;
    for c in s.chars().into_iter() {
        if c.eq(&'(') {
            score_buffer = 1;
            multiplier += 1;
        } else {
            total += score_buffer * multiplier;
            score_buffer = 0;
            multiplier -= 1;
        }
    }
    total
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::eval_parentheses;

    fn dotest(s: &str, expected: u32) {
        let actual = eval_parentheses(s);
        assert_eq!(actual, expected, "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("()", 1);
        dotest("(())", 2);
        dotest("()()", 2);
        dotest("((())())", 6);
        dotest("(()(()))", 6);
        dotest("()(())", 3);
        dotest("()((()))", 5);
    }
}
