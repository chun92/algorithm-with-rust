fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calculate_cuts(vec: &Vec<u64>, n: u64) -> u64 {
    let mut cuts = 0;
    for i in 0..vec.len() {
        cuts += vec[i] / n;
    }
    cuts
}

fn get_result(vec: &Vec<u64>, n: u64, ans: u64, min: u64, max: u64) -> u64 {
    if min > max {
        return ans;
    }

    let mid = (min + max) / 2;
    if calculate_cuts(vec, mid) >= n {
        get_result(vec, n, std::cmp::max(ans, mid), mid + 1, max)
    } else {
        get_result(vec, n, ans, min, mid - 1)
    }
}

fn main() {
    let args = read_line_as_numbers();
    let (k, n) = (args[0], args[1]);

    let mut targets = Vec::new();
    for _ in 0..k {
        let num = read_line_as_numbers()[0];
        targets.push(num);
    }
    let max = targets.iter().max().unwrap();
    let result = get_result(&targets, n, 0, 1, *max);
    println!("{}", result);
}
