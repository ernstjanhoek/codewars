pub fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|c|{
            match c {
                'T' => 'A',
                'A' => 'T',
                'G' => 'C',
                'C' => 'G',
                _ => 'N'
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert_eq!(actual, expected, "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
        dotest("AB", "TN");
    }
}