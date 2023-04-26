fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(memories: &Vec<i32>, costs: &Vec<i32>, n: i32, m: i32) -> i32 {
    let cost_sum = costs.iter().sum::<i32>();
    let mut memo = vec![vec![0; cost_sum as usize]; n as usize];

    let mut start = 0;
    let mut end = cost_sum;
    let mut mid;

    while start < end {
        mid = (start + end) / 2;
        let current = dp(&mut memo, &memories, &costs, n as usize - 1, mid);
    
        if current >= m {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    
    mid = (start + end) / 2;
    mid
}

fn dp(memo: &mut Vec<Vec<i32>>, memories: &Vec<i32>, costs: &Vec<i32>, i: usize, c: i32) -> i32 {
    if i == 0 {
        if c < costs[0] {
            return -1;
        } else {
            return memories[0];
        }
    }

    if memo[i][c as usize] > 0 {
        return memo[i][c as usize];
    }

    let res1 = dp(memo, memories, costs, i - 1, c);
    let res2 = if c >= costs[i] {
        dp(memo, memories, costs, i - 1, c - costs[i]) + memories[i]
    } else {
        -1
    };

    let res = std::cmp::max(res1, res2);
    memo[i][c as usize] = res;
    res
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1])
    };

    let memories = read_line_as_numbers();
    let costs = read_line_as_numbers();
    println!("{}", solve(&memories, &costs, n as i32, m));
}