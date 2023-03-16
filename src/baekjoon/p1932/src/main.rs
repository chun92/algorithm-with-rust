use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(n: u64, k: u64, vec: &Vec<Vec<u64>>, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if n == 0 {    
        if let Some(&v) = memo.get(&(n, k)) {
            return v;
        } else {
            memo.insert((0, 0), vec[0][0]);
            return vec[0][0];
        }
    }

    if let Some(&v) = memo.get(&(n, k)) {
        return v;
    }

    let case = {
        if k == n {
            dp(n - 1, k - 1, vec, memo) + vec[n as usize][k as usize]
        } else if k == 0 {
            dp(n - 1, k, vec, memo) + vec[n as usize][k as usize]
        } else {
            let case1 = dp(n - 1, k - 1, vec, memo) + vec[n as usize][k as usize];
            let case2 = dp(n - 1, k, vec, memo) + vec[n as usize][k as usize];
            std::cmp::max(case1, case2)
        }
    };

    memo.insert((n, k), case);
    case
}

fn result(n: u64,vec: &Vec<Vec<u64>>, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    let mut max = 0;
    for i in 0..=n {
        let case = dp(n, i, vec, memo);
        if case > max {
            max = case;
        }
    }
    max
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_numbers());
    }

    println!("{}", result(n - 1, &vec, &mut HashMap::new()));
}
