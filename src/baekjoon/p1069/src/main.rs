fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (x, y, d, t) = {
        let v = read_line_as_numbers();
        (v[0] as f64, v[1] as f64, v[2] as f64, v[3] as f64)
    };

    let mut distance = (x.powi(2) + y.powi(2)).sqrt();
    if d < t {
        println!("{}", distance);
    } else {
        let mut n = distance / d;
        let mut time = 0.0;
        if n >= 2.0 {
            let step = n.floor() - 1.0;
            time = step * t;
            distance -= step * d;
            n = n - step;
        }

        if n >= 1.0 {
            let time2 = t + (distance - d);
            let time3 = 2.0 * t;
            time += time2.min(time3);
        } else {
            let time1 = distance;
            let time2 = t + (d - distance);
            let time3 = 2.0 * t;
            time += time1.min(time2.min(time3));
        }

        println!("{}", time);
    }
}
