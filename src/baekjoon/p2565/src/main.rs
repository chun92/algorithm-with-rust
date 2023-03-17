fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(n: usize, vec: &Vec<(usize, usize)>) -> usize {
    if n == 1 {
        return 0;
    }

    let mut memo = Vec::new();
    memo.push(1);

    for i in 1..n {
        let mut max = 0;
        for j in 0..i {
            if vec[i].0 > vec[j].0 && vec[i].1 > vec[j].1 {
                max = std::cmp::max(max, memo[j]);
            }
        }
        memo.push(max + 1);
    }

    let max = memo.iter().max().unwrap();
    n - max
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        let line = read_line_as_numbers();
        vec.push((line[0], line[1]));
    }
    vec.sort_by_key(|&(a, _)| a);
    let result = solve(n, &vec);
    println!("{}", result);
}
