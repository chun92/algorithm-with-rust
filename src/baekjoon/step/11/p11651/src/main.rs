use std::io;
use io::Write;

fn read_lien_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn print_points(points: &Vec<(i32, i32)>) {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for p in points {
        writeln!(out, "{} {}", p.0, p.1).unwrap();
    }
}

fn main() {
    let n = read_lien_as_numbers()[0];
    let mut points = Vec::new();
    for _ in 0..n {
        let v = read_lien_as_numbers();
        points.push((v[0], v[1]));
    }

    points.sort_unstable_by(|a, b| {
        match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            ord => ord,
        }
    });

    print_points(&points);
}
