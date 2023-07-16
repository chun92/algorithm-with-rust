fn read_line_as_chars() -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.chars().collect()
}

fn main() {
    let mut sum = 0;
    for i in 0..8 {
        let vec = read_line_as_chars();
        for j in 0..8 {
            if (i + j) % 2 == 0 && vec[j] == 'F' {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
