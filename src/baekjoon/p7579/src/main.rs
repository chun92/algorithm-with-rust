fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(memories: &Vec<u32>, costs: &Vec<u32>, n: u32, m: u32) -> u32 {
    let cost_sum = costs.iter().sum::<u32>();
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

fn dp(memo: &mut Vec<Vec<u32>>, memories: &Vec<u32>, costs: &Vec<u32>, i: usize, c: u32) -> u32 {
    if i == 0 {
        if c < costs[0] {
            return 0;
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
        0
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
    println!("{}", solve(&memories, &costs, n as u32, m));
}