use std::i32::{MAX, MIN};

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_point(direction: i32, distance: i32, point: (i32, i32)) -> (i32, i32) {
    match direction {
        1 => (point.0 + distance, point.1),
        2 => (point.0 - distance, point.1),
        3 => (point.0, point.1 - distance),
        4 => (point.0, point.1 + distance),
        _ => (0, 0)
    }
}
fn main() {
    let n = read_line_as_numbers()[0];

    let mut points= Vec::new();
    for i in 0..6 {
        let nums = read_line_as_numbers();
        let previous = {
            if i == 0 {
                (0, 0)
            } else {
                points[i - 1]
            }
        };
        points.push(get_point(nums[0], nums[1], previous));
    }

    let mut min_x = MAX;
    let mut min_y = MAX;
    let mut max_x = MIN;
    let mut max_y = MIN;

    points
        .iter()
        .for_each(|(x, y)| {
            min_x = std::cmp::min(min_x, *x);
            min_y = std::cmp::min(min_y, *y);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        });
    
    let large_rect = (max_x - min_x) * (max_y - min_y);

    let mut new_min_x = MAX;
    let mut new_min_y = MAX;
    let mut new_max_x = MIN;
    let mut new_max_y = MIN;
    points
        .iter()
        .filter(|(x, y)| {
            match (*x, *y) {
                (x, y) if x == min_x && y == min_y => false,
                (x, y) if x == min_x && y == max_y => false,
                (x, y) if x == max_x && y == min_y => false,
                (x, y) if x == max_x && y == max_y => false,
                _ => true
            }
        })
        .for_each(|(x, y)| {
            new_min_x = std::cmp::min(new_min_x, *x);
            new_min_y = std::cmp::min(new_min_y, *y);
            new_max_x = std::cmp::max(new_max_x, *x);
            new_max_y = std::cmp::max(new_max_y, *y);
        });
    let small_rect: i32 = (new_max_x - new_min_x) * (new_max_y - new_min_y);
    let result = (large_rect - small_rect) * n;
    println!("{}", result);
}