pub fn is_pangram(s: &str) -> bool {
    let mut bitmask: u32 = 0;
    s.chars()
        .filter(|c|c.to_ascii_lowercase() >= 'a' && c.to_ascii_lowercase() <= 'z')
        .map(|c| (c.to_ascii_lowercase() as u8 - b'a') as u32)
        .for_each(|b|bitmask |= 1 << b);
    bitmask.eq(&67108863)
}

#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert_eq!(actual, expected, "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}
