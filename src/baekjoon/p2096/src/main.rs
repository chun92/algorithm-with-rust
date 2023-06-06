fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(min_memo: &mut Vec<usize>, max_memo: &mut Vec<usize>, n: usize) {
    let mut board = read_line_as_numbers();
    min_memo[0] = board[0];
    min_memo[1] = board[1];
    min_memo[2] = board[2];
    max_memo[0] = board[0];
    max_memo[1] = board[1];
    max_memo[2] = board[2];

    for _ in 1..n {
        let mut new_min_memo = vec![0; 3];
        let mut new_max_memo = vec![0; 3];
        board = read_line_as_numbers();
        new_min_memo[0] = board[0] + min_memo[0].min(min_memo[1]);
        new_min_memo[1] = board[1] + min_memo[0].min(min_memo[1]).min(min_memo[2]);
        new_min_memo[2] = board[2] + min_memo[1].min(min_memo[2]);
        new_max_memo[0] = board[0] + max_memo[0].max(max_memo[1]);
        new_max_memo[1] = board[1] + max_memo[0].max(max_memo[1]).max(max_memo[2]);
        new_max_memo[2] = board[2] + max_memo[1].max(max_memo[2]);
        
        min_memo[0] = new_min_memo[0];
        min_memo[1] = new_min_memo[1];
        min_memo[2] = new_min_memo[2];
        max_memo[0] = new_max_memo[0];
        max_memo[1] = new_max_memo[1];
        max_memo[2] = new_max_memo[2];
    }
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut min_memo = vec![0; 3];
    let mut max_memo = vec![0; 3];

    dp(&mut min_memo, &mut max_memo, n);
    println!("{} {}", max_memo.iter().max().unwrap(), min_memo.iter().min().unwrap());
}
