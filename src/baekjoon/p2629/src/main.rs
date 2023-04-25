use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(sources: &Vec<i32>, memo: &mut HashMap<(usize, i32), bool>, target: i32, index: usize) -> bool {
    let key = (index, target);
    if memo.contains_key(&key) {
        return memo[&key];
    }

    if target == 0 {
        return true;
    }

    if index == sources.len() {
        return false;
    }

    let dp1 = dp(sources, memo, target + sources[index], index + 1);
    let dp2 = dp(sources, memo, target, index + 1);
    let dp3 = dp(sources, memo, target - sources[index], index + 1);
    memo.insert(key, dp1 || dp2 || dp3);
    dp1 || dp2 || dp3
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let sources = read_line_as_numbers();
    let _m = read_line_as_numbers()[0];
    let targets = read_line_as_numbers();
    let mut memo = HashMap::new();

    let mut results = Vec::new();

    for target in targets {
        if dp(&sources, &mut memo, target, 0) {
            results.push("Y");
        } else {
            results.push("N");
        }
    }

    println!("{}", results.join(" "));
}
