fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let chikens = read_line_as_numbers()[0];
    let (cola, beer) = {
        let numbers = read_line_as_numbers();
        (numbers[0], numbers[1])
    };

    let possible = cola / 2 + beer;
    let result = std::cmp::min(chikens, possible);
    println!("{}", result);
}
