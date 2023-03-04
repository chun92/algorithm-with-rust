use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let vec :Vec<Vec<i32>> = (0..9).map(|_| read_line_as_numbers()).collect();

    let mut max = 0;
    let mut max_index = (0, 0);

    for (i, row) in vec.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if num > max {
                max = num;
                max_index = (i, j);
            }
        }
    }
    
    println!("{}", max);
    println!("{} {}", max_index.0 + 1, max_index.1 + 1);
}
