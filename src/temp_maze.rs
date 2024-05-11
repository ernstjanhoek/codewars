//NIET AF!

#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    N,
    W,
    S,
    E,
}
use Dir::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Value {
    Blocked,
    Open,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    value: Value,
    x: usize,
    y: usize,
    side: Option<Dir>,
    checked: bool, 
}


impl Point {
    fn new(value: Value, x: usize, y: usize) -> Self {
        Self {
            value,
            x,
            y,
            side: None,
            checked: false
        }
    }

    fn derive_side(mut self, max_n: usize) -> Self {
        if self.y == 0 {
            self.side = Some(N);
        }
        if self.x == 0 {
            self.side = Some(W);
        }
        if self.y == max_n {
            self.side = Some(S);
        }
        if self.x == max_n {
            self.side = Some(E);
        }
        self
    }

    fn checked(&self) -> bool {
        self.checked
    }

}

fn path_finder(maze: &str) -> bool {
    let out: Option<(usize, char)> = maze.chars().enumerate().find(|x| x.1 == '\n');
    println!("{:?}", out);
    let n = out.unwrap().0 - 1;
    println!("{:?}", n);
    false
}

fn old_path_finder(maze: &str) -> bool {
    let char_vec: Vec<char> = maze.chars().map(|c|c).collect();
    let mut out: Vec<Point> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut max_n: usize = 0;
    for (index, character) in char_vec.iter().enumerate() {
        match character {
            '.' | 'W' => {
                out.push(Point::new(*character, x, y));
                x += 1;
            },
            '\n' => {
                y += 1;
                x = 0;
                max_n = index;
            },
            _ => {
                panic!()
            }
        }
    }
    out = out.iter().map(|p|p.derive_side(max_n)).collect();


    for (index, point) in out.iter().enumerate() {
        if point.value == 'W' && point.side.is_some() {

        }
    }
    println!("{:?}", out);
    true
}

fn is_enclosing(side1: Dir, side2: Dir) -> bool {
    if side1 == side2 {
        return false
    }

    match (side1, side2) {
        (N, E) | (E, N) | (W, S) | (S, W) => return false,
        _ => return true,
    }
}


#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze("\
            .W.\n\
            .W.\n\
            ...\
            ",
            true,
            );

        test_maze("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
            true,
            );

        test_maze("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
            false,
            );
    }

    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
            Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
             actual,
             expect, 
             maze
             );
    }
}
