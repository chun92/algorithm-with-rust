fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn check_point_in_circle(x: i32, y: i32, cx: i32, cy: i32, r: i32) -> bool {
    let dx = x - cx;
    let dy = y - cy;
    dx * dx + dy * dy <= r * r
}

fn main() {
    let n = read_line_as_numbers()[0];

    for _ in 0..n {
        let targets = read_line_as_numbers();
        let (x1, y1, x2, y2) = (targets[0], targets[1], targets[2], targets[3]);
        let num = read_line_as_numbers()[0];
        let mut count = 0;
        for _ in 0..num {
            let planet = read_line_as_numbers();
            let (cx, cy, r) = (planet[0], planet[1], planet[2]);
            let in1 = check_point_in_circle(x1, y1, cx, cy, r);
            let in2 = check_point_in_circle(x2, y2, cx, cy, r);
            if in1 != in2 {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
