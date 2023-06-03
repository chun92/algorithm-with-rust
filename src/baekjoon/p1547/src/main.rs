fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let m = read_line_as_numbers()[0];
    let mut vec = (1..=3).collect::<Vec<usize>>();
    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        vec.swap(a - 1, b - 1);
    }
    println!("{}", vec.iter().position(|&x| x == 1).unwrap() + 1);
}
