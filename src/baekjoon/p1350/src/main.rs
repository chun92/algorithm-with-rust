fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();
    let chunk_size = read_line_as_numbers()[0];

    let mut result = 0;
    numbers
        .iter()
        .for_each(|x| {
            result += x / chunk_size * chunk_size;
            if x % chunk_size != 0 {
                result += chunk_size;
            }
        });
    
    println!("{}", result);
}
