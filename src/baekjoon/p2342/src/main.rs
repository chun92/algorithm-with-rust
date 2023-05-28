fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_step_cost(prev: usize, current: usize) -> usize {
    if prev == current {
        1
    } else if prev == 0 {
        2
    } else if current.abs_diff(prev) == 2 {
        4
    } else {
        3
    }
}

fn main() {
    let numbers = read_line_as_numbers();
    const MAX: usize = 5000000;
    let range = numbers.len() - 1;
    let mut memo = vec![vec![MAX; 5]; 5];

    let first_step = numbers[0];
    memo[0][first_step] = 2;
    memo[first_step][0] = 2;
    for i in 1..range {
        let current_step = numbers[i];
        let mut new_memo = vec![vec![MAX; 5]; 5];
        for j in 0..5 {
            for k in 0..5 {
                if memo[j][k] != MAX {
                    if k != current_step {
                        let cost = get_step_cost(j, current_step);
                        new_memo[current_step][k] = std::cmp::min(new_memo[current_step][k], memo[j][k] + cost);
                    }
                    if j != current_step {
                        let cost = get_step_cost(k, current_step);
                        new_memo[j][current_step] = std::cmp::min(new_memo[j][current_step], memo[j][k] + cost);
                    }
                }
            }
        }
        memo = new_memo;
    }

    let result = memo
        .iter()
        .map(|v| v.iter().min().unwrap())
        .min()
        .unwrap();
    println!("{}", result);
}
