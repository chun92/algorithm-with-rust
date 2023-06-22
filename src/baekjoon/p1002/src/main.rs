fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calculate_num_of_intersection(x1: f64, y1: f64, r1: f64, x2: f64, y2: f64, r2: f64) -> i32 {
    if x1 == x2 && y1 == y2 {
        if r1 == r2 {
            if r1 == 0.0 {
                1
            } else {
                -1
            }
        } else {
            0
        }
    } else if r1 == 0.0 {
        let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        if dist == r2 {
            1
        } else {
            0
        }
    } else if r2 == 0.0 {
        let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        if dist == r1 {
            1
        } else {
            0
        }
    } else {
        let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        
        if dist > r1 + r2 {
            0
        } else if dist == r1 + r2 {
            1
        } else if dist == (r2 - r1).abs() {
            1
        } else if dist < (r2 - r1).abs() {
            0
        } else {
            2
        }
    }
    
}
fn main() {
    let n = read_line_as_numbers()[0];
    for _ in 0..n {
        let arguments = read_line_as_numbers();
        let (x1, y1, r1, x2, y2, r2) = 
            (arguments[0] as f64, arguments[1] as f64, arguments[2] as f64, 
            arguments[3] as f64, arguments[4] as f64, arguments[5] as f64);
        println!("{}", calculate_num_of_intersection(x1, y1, r1, x2, y2, r2));
    }
}
