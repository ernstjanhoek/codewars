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
// Three 1's => 1000 points
// Three 6's =>  600 points
// Three 5's =>  500 points
// Three 4's =>  400 points
// Three 3's =>  300 points
// Three 2's =>  200 points
// One   1   =>  100 points
// One   5   =>   50 point
//noinspection ALL
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