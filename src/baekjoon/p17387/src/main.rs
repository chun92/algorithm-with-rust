use std::io::{self, BufRead};
use std::str::SplitWhitespace;

#[derive(Debug)]
struct Point {
    x: i128,
    y: i128,
}

fn read_numbers(line: &mut SplitWhitespace) -> (i128, i128) {
    let x: i128 = line.next().unwrap().parse().unwrap();
    let y: i128 = line.next().unwrap().parse().unwrap();
    (x, y)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i128 {
    let val = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if val > 0 {
        1
    } else if val < 0 {
        -1
    } else {
        0
    }
}

fn is_intersect(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    let ab = ccw(a, b, c) * ccw(a, b, d);
    let cd = ccw(c, d, a) * ccw(c, d, b);
    if ab == 0 && cd == 0 {
        return std::cmp::min(a.x, b.x) <= std::cmp::max(c.x, d.x)
            && std::cmp::min(c.x, d.x) <= std::cmp::max(a.x, b.x)
            && std::cmp::min(a.y, b.y) <= std::cmp::max(c.y, d.y)
            && std::cmp::min(c.y, d.y) <= std::cmp::max(a.y, b.y);
    }
    ab <= 0 && cd <= 0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut l1_line = line.split_whitespace();
    let (x1, y1) = read_numbers(&mut l1_line);
    let (x2, y2) = read_numbers(&mut l1_line);


    let line = lines.next().unwrap().unwrap();
    let mut l2_line = line.split_whitespace();
    let (x3, y3) = read_numbers(&mut l2_line);
    let (x4, y4) = read_numbers(&mut l2_line);

    let p1 = Point { x: x1, y: y1 };
    let p2 = Point { x: x2, y: y2 };
    let p3 = Point { x: x3, y: y3 };
    let p4 = Point { x: x4, y: y4 };

    println!("{}", if is_intersect(&p1, &p2, &p3, &p4) { 1 } else { 0 });
}
