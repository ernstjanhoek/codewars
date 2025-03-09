// https://www.codewars.com/kata/55ffb44050558fdb200000a4/train/rust

fn sum_dig_nth_term(init_val: u32, pattern: &[u32], nth: usize ) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::sum_dig_nth_term;

    #[test]
    fn sample_tests() {
        assert_eq!(sum_dig_nth_term(10, &[2, 1, 3], 6), 10);
        assert_eq!(sum_dig_nth_term(10, &[2, 1, 3], 15), 10);
        assert_eq!(sum_dig_nth_term(10, &[2, 1, 3], 50), 9);
        assert_eq!(sum_dig_nth_term(10, &[2, 1, 3], 78), 10);
        assert_eq!(sum_dig_nth_term(10, &[2, 1, 3], 157), 7);
        assert_eq!(sum_dig_nth_term(10, &[2, 2, 5, 8], 6), 11);
        assert_eq!(sum_dig_nth_term(10, &[2, 2, 5, 8], 15), 11);
        assert_eq!(sum_dig_nth_term(10, &[2, 2, 5, 8], 50), 9);
        assert_eq!(sum_dig_nth_term(10, &[2, 2, 5, 8], 78), 11);
        assert_eq!(sum_dig_nth_term(10, &[2, 2, 5, 8], 157), 16);
        assert_eq!(sum_dig_nth_term(100, &[2, 2, 5, 8], 6), 11);
        assert_eq!(sum_dig_nth_term(100, &[2, 2, 5, 8], 15), 11);
        assert_eq!(sum_dig_nth_term(100, &[2, 2, 5, 8], 50), 9);
        assert_eq!(sum_dig_nth_term(100, &[2, 2, 5, 8], 78), 11);
        assert_eq!(sum_dig_nth_term(100, &[2, 2, 5, 8], 157), 16);
    }
}
