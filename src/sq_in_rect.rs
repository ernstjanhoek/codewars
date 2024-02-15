fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    let mut out = Vec::new();
    let mut len = lng;
    let mut width = wdth;
    if len == width {
        None
    } else {
        loop { 
            out.push(width.min(len));
            let diff = len.abs_diff(width) as i32;
            len = len.min(width);
            width = diff;
            if diff == 0 {
                break
            }
        }
        Some(out)
    }
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {

    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);

}
