fn read_line_as_floats() -> Vec<f64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let liens = read_line_as_floats();
    let (x1, y1, r1) = (liens[0], liens[1], liens[2]);
    let (x2, y2, r2) = (liens[3], liens[4], liens[5]);
    let d = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();

    if d >= r1 + r2 {
        println!("{:.3}", 0.0);
    } else if d <= (r1 - r2).abs() {
        println!("{:.3}", std::f64::consts::PI * r1.min(r2).powf(2.0));
    } else {
        let theta1 = ((r1.powf(2.0) - r2.powf(2.0) + d.powf(2.0)) / (2.0 * r1 * d)).acos();
        let theta2 = ((r2.powf(2.0) - r1.powf(2.0) + d.powf(2.0)) / (2.0 * r2 * d)).acos();
        let s1 = r1.powf(2.0) * theta1 - theta1.sin() * r1 * d / 2.0;
        let s2 = r2.powf(2.0) * theta2 - theta2.sin() * r2 * d / 2.0;
        println!("{:.3}", s1 + s2);
    }
    
}