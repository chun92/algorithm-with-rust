fn ccw(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    (x2 - x1) * (y3 - y2) - (x3 - x2) * (y2 - y1)
}

fn read_line_as_point() -> (i32, i32) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    (x, y)
}

fn main() {
    let p1 = read_line_as_point();
    let p2 = read_line_as_point();
    let p3 = read_line_as_point();
    let ccw = ccw(p1, p2, p3);
    if ccw > 0 {
        println!("1");
    } else if ccw < 0 {
        println!("-1");
    } else {
        println!("0");
    }
}
