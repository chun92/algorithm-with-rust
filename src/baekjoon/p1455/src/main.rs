fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_0_and_1s() -> Vec<bool> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .chars()
        .map(|c| c == '1')
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut board = Vec::new();
    for _ in 0..n {
        board.push(read_line_as_0_and_1s());
    }

    let mut count = 0;
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if board[i][j] {
                count += 1;
                for k in 0..=i {
                    for l in 0..=j {
                        board[k][l] = !board[k][l];
                    }
                }
            }
        }
    }
    println!("{}", count);
}
