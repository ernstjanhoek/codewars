// NIET AF!

pub fn smallest(n: i64) -> (i64, usize, usize) {
    let num_vec: Vec<u32> = n.to_string().chars().map(|c|c.to_digit(10).unwrap()).collect();
    println!("{:?}", num_vec);
    let prom_small = find_prominent_small(&num_vec, 0, 0, 0);
    println!("{:?}", prom_small);
    (0, 0, 0)
}

fn helper(num_vec: &[u32]) {

}

fn find_prominent_small(num_vec: &[u32], offset: usize, start_index: usize, end_index: usize) -> usize {
    let mut result = num_vec[offset..].iter().enumerate()
        .filter(|(i, _)|*i < start_index || *i >= end_index)
        .min_by_key(|(_, &x)|x).unwrap();
    if result.0.eq(&offset) {
        result.0 = find_prominent_small(&num_vec, result.0 + 1, 0, 0);
    }
    if result.1.eq(&num_vec[result.0 + 2]) {
        println!("index: {} index2: {}", result.0, result.0 +1 );
        result.0 = find_prominent_small(&num_vec, 0, result.0, result.0 + 1);
    }
    result.0 + offset
}

fn find_prominent_big(num_vec: &Vec<u32>) -> (usize, &u32) {
    num_vec.iter().enumerate().max_by_key(|(_, &x)|x).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: (i64, usize, usize)) -> () {
        let ans = smallest(n);
        assert_eq!(ans, exp, "Testing: {}", n);
    }

    #[test]
    fn basic_tests() {
        testing(261235, (126235, 2, 0));
        testing(209917, (29917, 0, 1));
        testing(285365, (238565, 3, 1));
    }
}

