fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(graph: &Vec<Vec<bool>>, memo: &mut Vec<Vec<Vec<usize>>>, n: usize) {
    // direction
    // 0: right
    // 1: down
    // 2: right-down
    for i in 0..n {
        for j in 0..n {
            if !graph[i][j] {
                continue;
            }
            let cur_0 = memo[i][j][0];
            let cur_1 = memo[i][j][1];
            let cur_2 = memo[i][j][2];

            if i < n - 1 && graph[i + 1][j] {
                memo[i + 1][j][1] += cur_1 + cur_2;
            }

            if j < n - 1 && graph[i][j + 1] {
                memo[i][j + 1][0] += cur_0 + cur_2;
            }

            if i < n - 1 && j < n - 1 && graph[i + 1][j + 1] && graph[i + 1][j] && graph[i][j + 1] {
                memo[i + 1][j + 1][2] += cur_0 + cur_1 + cur_2;
            }
        }
    }
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut graph = vec![vec![false; n]; n];
    for i in 0..n {
        let line = read_line_as_numbers();
        for j in 0..n {
            graph[i][j] = line[j] == 0;
        }
    }

    let mut memo = vec![vec![vec![0; 3]; n]; n];
    memo[0][1][0] = 1;
    dp(&graph, &mut memo, n);
    let result: usize = memo[n - 1][n - 1].iter().sum();
    println!("{}", result);
}
