mod solution {
    pub fn range_extraction(a: &[i32]) -> String {
        // Your solution here
        let mut string = String::new();
        let mut peek_iter = a.iter().peekable();
        let mut range_buf: [Option<i32>; 2] = [Some(a[0]), None];
        while peek_iter.peek().is_some() {
            if let Some(num) = peek_iter.next() {
                if let Some(peek) = peek_iter.peek() {
                    if !is_consecutive(&num, &peek) {
                        if let Some(buf_str) = buf_to_string(&range_buf) {
                            string.push_str(buf_str.as_str());
                            range_buf = [None, None];
                        } else {
                            string.push_str(format!("{},", num).as_str());
                        }
                    } else {
                        range_buf = update_buf(range_buf, *num, **peek);
                    }
                } else {
                    if let Some(buf_str) = buf_to_string(&range_buf) {
                        string.push_str(buf_str.as_str()); 
                    } else {
                        string.push_str(format!("{},", num).as_str());
                    }
                }
            }
        }
        string.trim_end_matches(',').to_string()
    }

    fn is_consecutive(a: &i32, b:&i32) -> bool {
        b - a == 1
    }

    fn update_buf(old_buf: [Option<i32>; 2], num: i32, peek: i32) -> [Option<i32>; 2] {
        if let (Some(num), Some(_)) = (old_buf[0], old_buf[1]) {
            return [Some(num), Some(peek)];
        } else {
            return [Some(num), Some(peek)];
        }
    }

    fn buf_to_string(range_buf: &[Option<i32>; 2]) -> Option<String> {
        if let (Some(lower), Some(upper)) = (range_buf[0], range_buf[1]) {
            if !is_consecutive(&lower, &upper) {
                Some(format!("{}-{},", lower, upper))
            } else {
                Some(format!("{},{},", lower, upper))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");
        assert_eq!(solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]), "-3--1,2,10,15,16,18-20");
    }
}

