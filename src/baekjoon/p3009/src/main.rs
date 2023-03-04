use std::collections::HashMap;
fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();

    for _ in 0..3 {
        let numbers = read_line_as_numbers();
        let (x, y) = (numbers[0], numbers[1]);
        x_map.entry(x).and_modify(|v| *v += 1).or_insert(1);
        y_map.entry(y).and_modify(|v| *v += 1).or_insert(1);
    }
    x_map.iter().for_each(|(k, v)| {
        if *v == 1 {
            print!("{} ", k);
        }
    });
    y_map.iter().for_each(|(k, v)| {
        if *v == 1 {
            println!("{}", k);
        }
    });
}
