// https://www.codewars.com/kata/5270d0d18625160ada0000e4/train/rust/6640b305eb4723e9c618f3f4

use std::collections::HashMap;

pub fn score(dice: [u8; 5]) -> u32 {
    let mut score = 0;
    let mut counts = HashMap::new();

    for &num in &dice {
        *counts.entry(num).or_insert(0) += 1;
    }

    for (num, count) in &mut counts {
        if count >= &mut 3 {
            match num {
                1 => score += 1000,
                2 | 3 | 4 | 5 | 6 => score += u32::from(*num) * 100,
                _ => {}
            }
        }
    }

    for (_, count) in &mut counts {
        if *count >= 3 {
            *count -= 3;
        }
    }

    for (num, count) in &mut counts {
        if count >= &mut 1 {
            match num {
                1 => score += 100 * *count,
                5 => score += 50 * *count,
                _ => {}
            }
        }
    }

    score
}
#[cfg(test)]
mod tests {
    use super::score;

    fn dotest(dice: [u8; 5], expected: u32) {
        let actual = score(dice);
        assert!(actual == expected, "Expected score with dice {dice:?} to be {expected}, but was {actual}\n");
    }

    #[test]
    fn sample_tests() {
        dotest([2, 3, 4, 6, 2], 0);
        dotest([4, 4, 4, 3, 3], 400);
        dotest([2, 4, 4, 5, 4], 450);
    }
}