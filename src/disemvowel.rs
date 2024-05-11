fn disemvowel(s: &str) -> String {
    s.chars().filter_map(|value|match value {
        'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => None,
        _ => Some(value)}).collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}