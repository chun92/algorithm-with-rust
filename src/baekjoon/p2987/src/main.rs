use std::io;
use std::f64;

fn triangle_area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> f64 {
    ((x1*y2 + x2*y3 + x3*y1) - (y1*x2 + y2*x3 + y3*x1)).abs() as f64 / 2.0
}

fn main() {
    let mut input = String::new();
    let mut points = Vec::new();
    for _ in 0..3 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    let total_area = triangle_area(points[0].0, points[0].1, points[1].0, points[1].1, points[2].0, points[2].1);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut tree_count = 0;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        let sum_area = triangle_area(points[0].0, points[0].1, points[1].0, points[1].1, x, y) +
                        triangle_area(points[0].0, points[0].1, x, y, points[2].0, points[2].1) +
                        triangle_area(x, y, points[1].0, points[1].1, points[2].0, points[2].1);
        if (sum_area - total_area).abs() < f64::EPSILON {
            tree_count += 1;
        }
    }

    println!("{:.1}", total_area);
    println!("{}", tree_count);
}
