fn ccw(p1: (i128, i128), p2: (i128, i128), p3: (i128, i128)) -> i128 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    (x2 - x1) * (y3 - y2) - (x3 - x2) * (y2 - y1)
}

fn read_line_as_numbers() -> Vec<i128> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let line = read_line_as_numbers();
    let p1 = (line[0], line[1]);
    let p2 = (line[2], line[3]);
    let line = read_line_as_numbers();
    let p3 = (line[0], line[1]);
    let p4 = (line[2], line[3]);

    let d1 = ccw(p1, p2, p3);
    let d2 = ccw(p1, p2, p4);
    let d3 = ccw(p3, p4, p1);
    let d4 = ccw(p3, p4, p2);

    if d1 * d2 < 0 && d3 * d4 < 0 {
        println!("1");
    } else {
        println!("0");
    }
}
