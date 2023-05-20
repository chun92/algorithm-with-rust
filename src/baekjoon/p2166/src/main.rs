use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut coordinates: Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i64 = iter.next().unwrap().parse().unwrap();
        let y: i64 = iter.next().unwrap().parse().unwrap();
        coordinates.push((x, y));
    }

    let mut area = 0.0;
    for i in 0..n {
        let j = (i+1)%n;
        area += (coordinates[i].0 * coordinates[j].1 - coordinates[j].0 * coordinates[i].1) as f64;
    }
    area = (area / 2.0).abs();

    println!("{:.1}", area);
}
