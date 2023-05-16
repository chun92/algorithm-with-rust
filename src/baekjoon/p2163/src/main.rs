fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(i: u32, j: u32) -> u32 {
    if i != 1 {
        if i % 2 == 0 {
            return 2 * dp(i / 2, j) + 1;
        } else {
            return dp(i / 2, j) + dp(i / 2 + 1, j) + 1;
        }
    } else if j != 1 {
        if j % 2 == 0 {
            return 2 * dp(i, j / 2) + 1;
        } else {
            return dp(i, j / 2) + dp(i, j / 2 + 1) + 1;
        }
    } else {
        return 0;
    }
}

fn main() {
    let (i, j) = {
        let args = read_line_as_numbers();
        (args[0], args[1])
    };

    println!("{}", dp(i, j));
}
