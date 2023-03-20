fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let mut vec = read_line_as_numbers();
    vec.sort();
    
    let mut sum = 0;
    let vec: Vec<i32> = vec
        .iter()
        .map(|&x| {
            sum += x;
            sum
        })
        .collect();

    let result = vec.iter().sum::<i32>();
    println!("{}", result);
}
