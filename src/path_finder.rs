#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    NorthEastHalf,
    SouthWestHalf,
    Corner,
}
use Dir::*;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    value: Value,
    index: usize,
    side: Option<Dir>,
    checked: bool, 
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Value {
    Blocked,
    Open,
}
use Value::*;

impl Point {
    fn new(value: Value, index: usize) -> Self {
        Self {
            value,
            index: index,
            side: None,
            checked: false,
        }
    }

    fn derive_side(mut self, max_n: usize) -> Self {
        if self.index == 0 || max_n.pow(2) == self.index {
            self.side = Some(Corner);
            self
        } else { 
            if self.index < max_n {
                self.side = Some(NorthEastHalf);
            }
            if (self.index + 1) % max_n == 0 {
                self.side = Some(NorthEastHalf);
            }
            if self.index % max_n == 0 {
                self.side = Some(SouthWestHalf);
            }
            if self.index >= max_n.pow(2) - max_n && self.index < max_n.pow(2) {
                self.side = Some(SouthWestHalf);
            }
            self
        }
    }

    fn is_checked(&self) -> bool {
        self.checked
    }

}

fn derive_point_vec(maze: &str, max_n: usize) -> Vec<Point> {
    let mut point_vec = Vec::new();
    let mut index: usize = 0;
    for character in maze.chars() {
        match character {
            '.' | 'W' => {
                let value = if character == '.' {
                    Open
                } else {
                    Blocked
                };
                point_vec.push(Point::new(value, index).derive_side(max_n));
                index += 1;
            },
            '\n' => {
                ();
            },
            _ => {
                panic!()
            }
        }
    }
    point_vec
}

fn path_finder(maze: &str) -> bool {
    let max_n = maze.chars().position(|p|p == '\n'); 
    if let Some(max_n) = max_n {
        let mut point_vec: Vec<Point> = derive_point_vec(&maze, max_n);
        return find_path_block(&mut point_vec, max_n);
    } if let None = max_n {
        true 
    } else {
        false
    }
}

fn find_path_block(point_vec: &mut Vec<Point>, max_n: usize) -> bool {
    let mut side_buffer = Vec::new();
    for point in point_vec.as_slice().iter() {
        if point.value == Blocked && point.side.is_some() && !point.is_checked() {
            side_buffer.push(*point);
        }
    }

    let mut buffer = Vec::new();
    let mut start_dir: Option<Dir>;
    while !side_buffer.is_empty() {
        let point = side_buffer.remove(0);
        point_vec[point.index].checked = true;
        let neighbour_candidates = map_neighbours(&point_vec.as_slice(), point.index, max_n);
        let mut neighbours = neighbour_candidates.check_neighbours();
        buffer.append(&mut neighbours);
        start_dir = point.side;
        while !buffer.is_empty() {
            let point = buffer.remove(0);
            point_vec[point.index].checked = true;
            let neighbour_candidates = map_neighbours(&point_vec.as_slice(), point.index, max_n); 
            let mut neighbours = neighbour_candidates.check_neighbours();  
            buffer.append(&mut neighbours);
            if let Some(side) = point.side {
                if is_enclosing(start_dir.unwrap(), side) {
                    return false
                }
            }
        }
    }
    true
}

fn map_neighbours(slice: &[Point], point_index: usize, n: usize) -> Neighbours {
    let mut points: Vec<Point> = Vec::new();

    let point_index = point_index as i32;
    let n = n as i32;
    let ln = slice.len() as i32;
    let lower = if point_index % n == 0 { point_index } else { point_index - 1 }; 
    let upper = if point_index % n == n - 1 { point_index } else { point_index + 1 };
    let ranges: Vec<(usize, usize)> = vec![

        (
            std::cmp::max(0, lower - n).try_into().unwrap(), 
            std::cmp::max(0, upper - n).try_into().unwrap()
        ),  
        (
            std::cmp::max(0, lower).try_into().unwrap(), 
            std::cmp::min(ln, upper).try_into().unwrap()
        ),  
        (
            std::cmp::min(ln, lower + n).try_into().unwrap(), 
            std::cmp::min(ln, upper + n).try_into().unwrap()
        ),
    ];
    for range in ranges {
        if let Some(range_slice) = slice.get(range.0..=range.1) {
            for point in range_slice {
                points.push(*point);
            }
        }
    }
    Neighbours { points }
}

#[derive(Clone, Debug, PartialEq)]
struct Neighbours {
    points: Vec<Point>,
}

// checks the points in slice and returns vec of blocked points.  
impl Neighbours {
    fn check_init_neighbours(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for point in &self.points {
            if point.value == Blocked && !point.side.is_some() {
                points.push(*point)
            }
        }
        points
    }

    fn check_neighbours(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for point in &self.points {
            if point.value == Blocked && !point.is_checked() {
                points.push(*point)
            }
        }
        points
    }
}

fn is_enclosing(side1: Dir, side2: Dir) -> bool {
    if side1 == side2 {
        return false
    } else {
        return true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test_maze("\
        .....W.\n\
        W......\n\
        .W.....\n\
        ..WWWWW\n\
        .......\n\
        W......\n\
        ...W...\
        ",
        false,
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
