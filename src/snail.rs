#[derive(Copy, Clone, Debug)]
enum Dir {
    E,
    S,
    W,
    N,
}
use Dir::*;

impl Dir {
    fn change(self)-> Self {
        match self {
            E => S,
            S => W,
            W => N,
            N => E,
        }
    }
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut out_vec: Vec<i32> = Vec::new();
    let mut dir = Dir::E;
    let mut count = 0;
    let mut n = matrix.len();
    while n > 0 && !matrix[0].is_empty() {
        let mut values = extract_values(n, dir, matrix);
        println!("{:?}", values);
        out_vec.append(&mut values);
        n = decrease_n(count, n);
        println!("{:?}", n);
        dir = dir.change();
        count += 1;
    }
    out_vec
}

fn decrease_n(index: usize, n: usize) -> usize {
    if index % 2 == 0 {
        n - 1
    } else {
        n
    }
}

fn extract_values(n: usize, dir: Dir, slice: &[Vec<i32>]) -> Vec<i32> {
    let mut count = 0;
    let len_index = slice[0].len() - 1; // convert to index which starts from 0!
    let n_index = n - 1; // convert n to index (from 0)
    let mut out_vec: Vec<i32> = Vec::new();
    match dir {
        E => {
            let xy = (len_index - n_index) / 2;
            while count < n {
                out_vec.push(slice[xy][xy+count]);
                count += 1;
            }
        },
        S => {
            let y = ((len_index - n_index) / 2) + 1;
            let x = len_index - (len_index - n_index) / 2;
            println!("n: {}, dir: {:?}, x: {}, y: {}",n, dir, x ,y);
            while count < n {
                out_vec.push(slice[y+count][x]);
                count += 1;
            }
        },
        W => {
            let y = len_index - ((len_index - n_index) / 2);
            let x = len_index - ((len_index - n_index) / 2) - 1;
            println!("n: {}, dir: {:?}, x: {}, y: {}",n, dir, x ,y);
            while count < n {
                out_vec.push(slice[y][x-count]);
                count += 1;
            }

        },
        N => {
            let y = len_index - (len_index - n_index) / 2;
            let x = (len_index - n_index - 1) / 2;
            println!("n: {}, dir: {:?}, x: {}, y: {}",n, dir, x ,y);
            while count < n {
                out_vec.push(slice[y-count][x]);
                count += 1;
            }

        },
    }
    out_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn sample_test0() {
        let square = &[
            vec![1,  2,  3,  4,  5], 
            vec![6,  7,  8,  9,  10], 
            vec![11, 12, 13, 14, 15], 
            vec![16, 17, 18, 19, 20], 
            vec![21, 22, 23, 24, 25],
        ];
            let expected = vec![1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12, 13];
            assert_eq!(snail(square), expected);
    }
    //    #[test]
    //    fn sample_test1() {
    //        let square = &[
    //            vec![1,2,3],
    //            vec![4,5,6],
    //            vec![7,8,9],
    //        ];
    //            let expected = vec![1,2,3,6,9,8,7,4,5];
    //            assert_eq!(snail(square), expected);
    //    }
    //
    //    #[test]
    //    fn sample_test2() {
    //      let square = &[
    //            vec![1,2,3],
    //            vec![8,9,4],
    //            vec![7,6,5],
    //        ];
    //            let expected = vec![1,2,3,4,5,6,7,8,9];
    //            assert_eq!(snail(square), expected);
    //    }
    //
    //    #[test]
    //    fn sample_test3() {
    //        let square: &[Vec<i32>; 1] = &[Vec::new()];
    //        let expected = Vec::new();
    //        assert_eq!(snail(square), expected, "Failed with empty input");
    //    }
    //
    //    #[test]
    //    fn sample_test4() {
    //        let square = &[vec![1]];
    //        let expected = vec![1];
    //        assert_eq!(snail(square), expected);
    //    }
}
