fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn dfs(graph: &Vec<Vec<char>>, set: &mut Vec<bool>, r: usize, c: usize, cur: usize, max: &mut usize) {
    let alphabet = graph[r][c];
    let alphabet = alphabet as usize - 'A' as usize;
    if set[alphabet] {
        return;
    }

    if max == &26 {
        return;
    }

    set[alphabet] = true;
    let cur = cur + 1;
    *max = std::cmp::max(cur, *max);   

    if r > 0 {
        dfs(graph, set, r - 1, c, cur, max);
    }
    if r < graph.len() - 1 {
        dfs(graph, set, r + 1, c, cur, max);
    }
    if c > 0 {
        dfs(graph, set, r, c - 1, cur, max);
    }
    if c < graph[0].len() - 1 {
        dfs(graph, set, r, c + 1, cur, max);
    }
    
    set[alphabet] = false;
}

fn main() {
    let (r, c) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = vec![vec![' '; c]; r];

    for i in 0..r {
        let line = read_line_as_string();
        for (j, c) in line.chars().enumerate() {
            graph[i][j] = c;
        }
    }

    let mut set = vec![false; 26];
    let mut max = 0;
    dfs(&graph, &mut set, 0, 0, 0, &mut max);
    println!("{}", max);
}
