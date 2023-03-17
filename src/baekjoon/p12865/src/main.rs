use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(n: i32, w: i32, memo: &mut HashMap<(i32, i32), i32>, items: &Vec<(i32, i32)>) -> i32 {
    if let Some(&v) = memo.get(&(n, w)) {
        return v;
    }

    if n == 0 {
        return 0;
    }

    let (wn, vn) = items[n as usize - 1];

    let result = {
            if wn > w {
            solve(n - 1, w, memo, items)
        } else {
            let v1 = solve(n - 1, w, memo, items);
            let v2 = solve(n - 1, w - wn, memo, items) + vn;
            if v1 > v2 {
                v1
            } else {
                v2
            }
        }
    };
    memo.insert((n, w), result);
    result
}

fn main() {
    let args = read_line_as_numbers();
    let (n, w) = (args[0], args[1]);
    
    let mut items = Vec::new();
    for _ in 0..n {
        let args = read_line_as_numbers();
        items.push((args[0], args[1]));
    }

    let mut memo = HashMap::new();
    let result = solve(n, w, &mut memo,&items);
    println!("{}", result);
}
