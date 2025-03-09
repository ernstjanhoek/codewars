use std::{collections::HashMap, ops::Add};

#[derive(Debug, Default)]
struct RomanNumerical {
    pub symbol: char,
    pub value: i32,
}

impl RomanNumerical {
    pub fn new(symbol: char, value: i32) -> Self {
        Self {
            symbol,
            value
        }
    }

    pub fn decrements_next(&self, next: &RomanNumerical) -> bool {
        self.value < next.value
    }

    pub fn calculate_difference(&self, next: &RomanNumerical) -> i32 {
        next.value - self.value
    }
}

pub fn roman_as_num(roman: &str) -> u64 {
    let roman_map = HashMap::from([ 
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
        ]);
        
    let mut roman_iter = roman.chars().peekable();
    let mut sum: i32 = 0;

    while let Some(ch) = roman_iter.next() {
        let current_number = roman_map.get(&ch);
        
        let ch_next = roman_iter.peek();

        let current_roman = RomanNumerical::new(ch, *current_number.expect("no number without char"));

        let next_roman = {
            if let Some(&ch_next) = ch_next {
                RomanNumerical::new(ch_next, *roman_map.get(&ch_next).expect("no number without char"))
            } else {
                RomanNumerical::default()
            }
        };

        if current_roman.decrements_next(&next_roman) {
            sum = sum.add(current_roman.calculate_difference(&next_roman));
            roman_iter.next();
        } else {
            sum = sum.add(current_roman.value);
        }
    }
    return sum.try_into().unwrap();
}

#[cfg(test)]
mod roman_as_num_tests {
    use super::roman_as_num;

    fn test_equal(input: &str, actual: u64, expected: u64) {
        assert_eq!(actual, expected, "\nYour result (left) did not match the expected output (right) for the input \"{}\"", input);
    }
    
    #[test]
    fn basic() {
        test_equal("", roman_as_num(""), 0);
        test_equal("I",roman_as_num("I"), 1);
        test_equal("XXI", roman_as_num("XXI"), 21);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MDCLXVI", roman_as_num("MDCLXVI"), 1666);
    }
    
    #[test]
    fn one_through_ten() {
        test_equal("I", roman_as_num("I"), 1);
        test_equal("II", roman_as_num("II"), 2);
        test_equal("III", roman_as_num("III"), 3);
        test_equal("IV", roman_as_num("IV"), 4);
        test_equal("V", roman_as_num("V"), 5);
        test_equal("VI", roman_as_num("VI"), 6);
        test_equal("VII", roman_as_num("VII"), 7);
        test_equal("VIII", roman_as_num("VIII"), 8);
        test_equal("IX", roman_as_num("IX"), 9);
        test_equal("X", roman_as_num("X"), 10);
    }
    
    #[test]
    fn big_numbers() {
        test_equal("C", roman_as_num("C"), 100);
        test_equal("CDXLIV", roman_as_num("CDXLIV"), 444);
        test_equal("M", roman_as_num("M"), 1000);
        test_equal("MCMLIV", roman_as_num("MCMLIV"), 1954);
        test_equal("MCMXC", roman_as_num("MCMXC"), 1990);
        test_equal("MM", roman_as_num("MM"), 2000);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MMM", roman_as_num("MMM"), 3000);
        test_equal("MMMCM", roman_as_num("MMMCM"), 3900);
        test_equal("MMMCMXIV", roman_as_num("MMMCMXIV"), 3914);
    }    
}
