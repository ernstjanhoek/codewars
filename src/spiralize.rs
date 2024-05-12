// https://www.codewars.com/kata/534e01fbbb17187c7e0000c6/train/rust/663e6dd7d5a4c1e24bd9f5af

pub struct PointData {
    current: Corner,
    max_x: i8,
    max_y: i8,
    min_x: i8,
    min_y: i8,
}

impl PointData {
    fn new(grid_size: i8) -> Self {
        Self {
            current: UpperLeft { x: 0, y: 0 },
            max_x: grid_size,
            max_y: grid_size,
            min_x: 0,
            min_y: 0,
        }
    }

    fn traverse_point(&mut self, spiral_width: i8, x: i8, y: i8) -> Self {
        match &self.current {
            UpperLeft { .. } => { 
                // left upper corner, go right, increase min_y!
                Self {
                    current: self.current.traverse(x, y),
                    max_x: self.max_x,
                    max_y: self.max_y,
                    min_x: self.min_x,
                    min_y: self.min_y + spiral_width,
                }
            },
            UpperRight { .. } => {
                // right upper corner, go down, decrease, max_x!
                Self {
                    current: self.current.traverse(x, y),
                    max_x: self.max_x - spiral_width,
                    max_y: self.max_y,
                    min_x: self.min_x,
                    min_y: self.min_y,
                }
            },
            LowerRight { .. } => {
                // right lower corner, go left, decrease_max_y!
                Self {
                    current: self.current.traverse(x, y),
                    max_x: self.max_x,
                    max_y: self.max_y - spiral_width,
                    min_x: self.min_x,
                    min_y: self.min_y,
                }
            },
            LowerLeft { .. } => {
                // left lower corner, go up, increase min_x!
                Self {
                    current: self.current.traverse(x, y),
                    max_x: self.max_x,
                    max_y: self.max_y,
                    min_x: self.min_x + spiral_width,
                    min_y: self.min_y,
                }
            },
        }
    }

    fn return_next_xy(&self) -> (i8, i8) {
        match self.current {
            UpperLeft { .. } => (self.max_x, self.min_y),
            UpperRight { .. } => (self.max_x, self.max_y), 
            LowerRight { .. } => (self.min_x, self.max_y), 
            LowerLeft { .. } => (self.min_x, self.min_y),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Corner {
    UpperLeft { x: i8, y: i8 },
    UpperRight { x: i8, y: i8 },
    LowerRight { x: i8, y: i8 },
    LowerLeft { x: i8, y: i8 },
}
use Corner::*;

impl Corner {
    fn traverse(self, x: i8, y: i8) -> Self {
        match self {
            UpperLeft { .. } => UpperRight { x, y },
            UpperRight { .. } => LowerRight { x, y },
            LowerRight { .. } => LowerLeft { x, y },
            LowerLeft { .. } => UpperLeft { x, y },
        }
    }
    fn return_xy(&self) -> (i8, i8) {
        match self {
            UpperLeft { x, y } => (*x, *y),
            UpperRight { x, y } => (*x, *y),
            LowerRight { x, y } => (*x, *y),
            LowerLeft { x, y } => (*x, *y),
        }
    }
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let size_to_index: i8 = (size - 1).try_into().unwrap();
    let mut vec = vec![vec![0i8; size];size];
    let mut point = PointData::new(size_to_index);
    let mut abs_next_xy = point.return_next_xy();
    let mut abs_cur_xy: (i8, i8) = (0, 0);
    let mut range = {
        let old = point.current.return_xy();
        let new = point.return_next_xy();
        (new.0 - old.0, new.1 - old.1)
    };

    while point.max_y >= point.min_y || point.max_x >= point.min_x {
        if range.0.abs() > 0 || range.1.abs() > 0 {
            if abs_cur_xy.0 < abs_next_xy.0 {
                for i in abs_cur_xy.0..=abs_next_xy.0 {
                    vec[abs_cur_xy.1 as usize][i as usize] = 1;
                }
            } else if abs_cur_xy.0 > abs_next_xy.0 {
                for i in abs_next_xy.0..=abs_cur_xy.0 {
                    vec[abs_cur_xy.1 as usize][i as usize] = 1;
                }
            } else if abs_cur_xy.1 < abs_next_xy.1 {
                for i in abs_cur_xy.1..=abs_next_xy.1 {
                    vec[i as usize][abs_cur_xy.0 as usize] = 1;
                }
            } else if abs_cur_xy.1 > abs_next_xy.1 {
                for i in abs_next_xy.1..=abs_cur_xy.1 {
                    vec[i as usize][abs_cur_xy.0 as usize] = 1;
                }
            }
            abs_cur_xy = (abs_cur_xy.0 + range.0, abs_cur_xy.1 + range.1);
        }
        point = point.traverse_point(2i8, abs_next_xy.0, abs_next_xy.1);
        abs_next_xy = point.return_next_xy();
        range = { 
            let old = point.current.return_xy();
            let new = point.return_next_xy();
            (new.0 - old.0, new.1 - old.1)
        };
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    // 31 1 29 17 31
    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            ],
            );
    }
//[
//[1, 1, 1, 1, 1], 
//[0, 0, 0, 0, 1], 
//[1, 1, 1, 0, 1], 
//[0, 0, 1, 0, 1], 
//[1, 1, 1, 1, 1]
//]


    //[
    //[1, 1, 1, 1, 1], 
    //[0, 0, 1, 0, 1], 
    //[1, 1, 1, 0, 1], 
    //[0, 0, 0, 0, 1], 
    //[1, 1, 1, 1, 1]]
    //[
    //[1, 1, 1, 1, 1], 
    //[0, 0, 0, 0, 1], 
    //[1, 1, 1, 0, 1], 
    //[0, 0, 1, 0, 1], 
    //[1, 1, 1, 1, 0]]/
    

    //[1, 1, 1, 1, 1], 
    //[1, 0, 0, 0, 0], 
    //[1, 1, 1, 0, 0], 
    //[1, 0, 0, 0, 0], 
    //[1, 1, 1, 1, 1]]`,
        #[test]
        fn test8() {
            assert_eq!(
                spiralize(8),
                [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
                ],
                );
        }
        //[
        //[1, 1, 1, 1, 1, 1, 1, 1], 
        //[0, 0, 0, 0, 0, 0, 0, 1], 
        //[1, 1, 1, 1, 1, 1, 0, 1], 
        //[1, 0, 0, 0, 0, 1, 0, 1], 
        //[1, 0, 0, 0, 0, 1, 0, 1], 
        //[1, 0, 1, 1, 1, 1, 0, 1], 
        //[1, 0, 0, 0, 0, 0, 0, 1], 
        //[1, 1, 1, 1, 1, 1, 1, 1]]
        //
}

