// https://www.codewars.com/kata/5426006a60d777c556001aad/train/rust

/*
A pixmap shall be turned from black to white, turning all pixels to white in the process. But for optical reasons this shall not happen linearly, starting at the top and continuing line by line to the bottom:

for y in 0..height {
  for x in 0..width {
    set_bit(x, y);

Instead it shall be done by a systematic dithering routine which selects the coordinates on which pixels are to be set in a precise pattern so that the geometrical distance between two consecutive pixels is large and a specific optical effect results.

The following diagrams show the order of the coordinates the algorithm shall produce:

2×2:

1 3
4 2

4×4:

 1  9  3 11
13  5 15  7
 4 12  2 10
16  8 14  6

8×8:

 1  33   9  41   3  35  11  43
49  17  57  25  51  19  59  27
13  45   5  37  15  47   7  39
61  29  53  21  63  31  55  23
 4  36  12  44   2  34  10  42
52  20  60  28  50  18  58  26
16  48   8  40  14  46   6  38
64  32  56  24  62  30  54  22

The pattern continues like that for each square pixmap with a width and height of a power of two (16, 32, 64, …).

But the width and the height of the pixmap can be arbitrary positive numbers. If the pixmap's width and/or height are not a power of two, the coordinates the algorithm would produce outside of the pixmap are skipped:

3×3:

1 6 3
8 5 9
4 7 2

4x4
 1  9  3 11
13  5 15  7
 4 12  2 10
16  8 14  6

6×5:

 1 16  6 21  3 18
25 10 28 13 26 11
 8 23  5 20  9 24
29 14 27 12 30 15
 4 19  7 22  2 17

Write an algorithm which produces the coordinates of the dithering for a given width and height.

To pass the tests, write a function dithering which returns an iterator of tuples:

dithering(6, 5).take(6) -> (0, 0), (4, 4), (4, 0), (0, 4), (2, 2), (2, 0)
 */

pub fn dithering(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    let (mut width_cuts, mut height_cuts) = (vec![], vec![]);
    let (mut pow_two_width, mut pow_two_height) = (width.next_power_of_two(), height.next_power_of_two());

    while pow_two_width > 0 {
        pow_two_width = pow_two_width / 2;
        width_cuts.push(pow_two_width);
    }

    while pow_two_height > 0 {
        pow_two_height = pow_two_height / 2;
        height_cuts.push(pow_two_height);
    }
    for y in &height_cuts {
        for x in &width_cuts {
            print!("({}, {}) ",x, y);
        }
    }

    // traverse 0, +x+y, +x, +y
    // move origin
    // wrapping?
    println!("{:?}", width_cuts);
    println!("{:?}", height_cuts);

    // implement and return an iterator
    // Calculate the maximum dimension (width or height)
    let max_dim = width.max(height);
    // Calculate the number of cycles needed to cover the entire pixmap
    let cycles = (max_dim as f64).log2() as usize;

    // Generate the dithering pattern
    (0..cycles)
        .flat_map(move |cycle| {
            let step = 2usize.pow(cycle as u32 + 1);
            (0..step).flat_map(move |i| {
                (0..step).map(move |j| {
                    let x = (i * width) / step;
                    let y = (j * height) / step;
                    (x, y)
                })
            })
        })
        .filter(move |&(x, y)| x < width && y < height)
}


#[cfg(test)]
mod sample_tests {
    use super::dithering;

    #[test]
    fn partial_value() {
        let actual = dithering(4,4).nth(10);
        assert_eq!(actual, Some((3, 0)), "\ndithering(4,4): The 10th element in the iterator should be (3,0), instead got {actual:?}\n");
    }

    #[test]
    fn square_maps() {
        // 2 x 2
        let expected = [(0, 0), (1, 1), (1, 0), (0, 1)];
        let actual = dithering(2, 2).collect::<Vec<(usize,usize)>>();
        assert_eq!(actual, expected, "\ndithering(2,2): Your result (left) did not match expected output (right)");

        // 8 x 8
        let expected = [
            (0, 0),(4, 4),(4, 0),(0, 4),(2, 2),(6, 6),(6, 2),(2, 6),
            (2, 0),(6, 4),(6, 0),(2, 4),(0, 2),(4, 6),(4, 2),(0, 6),
            (1, 1),(5, 5),(5, 1),(1, 5),(3, 3),(7, 7),(7, 3),(3, 7),
            (3, 1),(7, 5),(7, 1),(3, 5),(1, 3),(5, 7),(5, 3),(1, 7),
            (1, 0),(5, 4),(5, 0),(1, 4),(3, 2),(7, 6),(7, 2),(3, 6),
            (3, 0),(7, 4),(7, 0),(3, 4),(1, 2),(5, 6),(5, 2),(1, 6),
            (0, 1),(4, 5),(4, 1),(0, 5),(2, 3),(6, 7),(6, 3),(2, 7),
            (2, 1),(6, 5),(6, 1),(2, 5),(0, 3),(4, 7),(4, 3),(0, 7),
        ];
        let actual = dithering(8, 8).collect::<Vec<(usize,usize)>>();
        assert_eq!(actual, expected, "\ndithering(8,8): Your result (left) did not match expected output (right)");
    }

    #[test]
    fn non_square_maps() {
        // 3 x 3
        let expected = [(0, 0), (2, 2), (2, 0), (0, 2), (1, 1), (1, 0), (1, 2), (0, 1), (2, 1)];
        let actual = dithering(3, 3).collect::<Vec<(usize,usize)>>();
        assert_eq!(actual, expected, "\ndithering(3,3): Your result (left) did not match expected output (right)");

        // 6 x 5
        let expected = [
            (0, 0),(4, 4),(4, 0),(0, 4),(2, 2),(2, 0),(2, 4),(0, 2),
            (4, 2),(1, 1),(5, 1),(3, 3),(3, 1),(1, 3),(5, 3),(1, 0),
            (5, 4),(5, 0),(1, 4),(3, 2),(3, 0),(3, 4),(1, 2),(5, 2),
            (0, 1),(4, 1),(2, 3),(2, 1),(0, 3),(4, 3),
        ];
        let actual = dithering(6, 5).collect::<Vec<(usize,usize)>>();
        assert_eq!(actual, expected, "\ndithering(6,5): Your result (left) did not match expected output (right)");
    }
}