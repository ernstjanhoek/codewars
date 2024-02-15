fn increment_string(s: &str) -> String {
    let mut string = s.to_string();
    let mut num_part = String::new();
    for chr in s.chars().rev() {
        if chr.is_numeric() {
            num_part.push(chr);
        } else {
            break
        }
    }
    println!("{:?}", num_part);
    if num_part.is_empty() {
        string.push('1');
        return string;
    } else {
        let (digits, str_part) = {
            (
                num_part.len(), 
                string.trim_end_matches(char::is_numeric).to_string()
            )
        };
        let mut overflow:Option<u8> = Some(1u8);
        let mut num_vec: Vec<u8> = num_part.as_bytes().to_vec().iter().map(|b|b - 48).collect();
        while overflow == Some(1) {
            for digit in num_vec.iter_mut() {
                if *digit != 9 {
                    *digit += 1;
                    overflow = None;
                    break; 
                } else {
                    *digit = 0;
                }
            }
            if let Some(1) = overflow {
                num_vec.push(1);
                overflow = None;
            }
        }
        let num_string: String = num_vec.iter().rev().map(|d|char::from_digit(u32::from(*d), 10).unwrap()).collect();
        return format!("{}{:0>digits$}",str_part, num_string);
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected, 
                "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        //dotest("HereCome9TrickyTests99999999999999999999999999999999999999", "HereCome9TrickyTests100000000000000000000000000000000000000");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("foo9bar9999", "foo9bar10000");
        dotest("", "1");
    }
}
