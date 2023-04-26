use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(memo: &mut HashMap<i64, i64>, from: i64, to: i64) -> i64 {
    if from > to {
        return -1;
    }

    if from == to {
        return 1;
    }

    if let Some(&count) = memo.get(&from) {
        return count;
    }

    let first = dp(memo, from * 2, to);
    let second = dp(memo, from * 10 + 1, to);

    let count = match (first, second) {
        (-1, -1) => -1,
        (-1, _) => second + 1,
        (_, -1) => first + 1,
        (_, _) => std::cmp::min(first, second) + 1,
    };

    memo.insert(from, count);
    count
}

fn main() {
    let mut memo = HashMap::new();

    let (x, y) = {
        let numbers = read_line_as_numbers();
        (numbers[0], numbers[1])
    };

    let result = dp(&mut memo, x, y);
    println!("{}", result);
}
