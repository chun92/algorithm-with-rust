use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dfs(numbers: &mut VecDeque<i64>, ops: &mut Vec<i64>, cur: &mut i64) -> (i64, i64) {
    if numbers.len() == 0 {
        return (*cur, *cur);
    }

    let mut max = i64::min_value();
    let mut min = i64::max_value();

    for i in 0..ops.len() {
        let op = ops[i];

        if op == 0 {
            continue;
        }
        
        let prev_cur = *cur;
        let num = numbers.pop_front().unwrap();

        match i {
            0 => *cur += num,
            1 => *cur -= num,
            2 => *cur *= num,
            3 => *cur /= num,
            _ => panic!("invalid operator")
        }
        
        ops[i] -= 1;

        let result = dfs(numbers, ops, cur);
        max = max.max(result.0);
        min = min.min(result.1);

        *cur = prev_cur;
        numbers.push_front(num);
        ops[i] += 1;
    }

    return (max, min);
}

fn main() {
    let _n = read_line_as_numbers();
    let mut numbers = read_line_as_numbers()
        .into_iter()
        .collect::<VecDeque<i64>>();
    let mut ops = read_line_as_numbers();

    let mut first_num = numbers.pop_front().unwrap();
    let (max, min) = dfs(&mut numbers, &mut ops, &mut first_num);
    println!("{}", max);
    println!("{}", min);
}
