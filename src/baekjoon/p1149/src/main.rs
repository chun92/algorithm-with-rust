use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(n: i32, vecs: &Vec<(i32, i32, i32)>, memo: &mut HashMap<i32, (i32, i32, i32)>) -> (i32, i32, i32) {
    if n == 0 {
        memo.insert(n, vecs[0]);
        return vecs[0];
    }

    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let (r_prev, g_prev, b_prev) = dp(n - 1, vecs, memo);
    let r = vecs[n as usize].0 + std::cmp::min(g_prev, b_prev);
    let g = vecs[n as usize].1 + std::cmp::min(r_prev, b_prev);
    let b = vecs[n as usize].2 + std::cmp::min(r_prev, g_prev);
    memo.insert(n, (r, g, b));

    (r, g, b)
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut vecs = Vec::new();
    for _ in 0..n {
        let v = read_line_as_numbers();
        vecs.push((v[0], v[1], v[2]));
    }
    let result = dp(n - 1, &vecs, &mut HashMap::new());
    println!("{}", std::cmp::min(result.0, std::cmp::min(result.1, result.2)));
}
