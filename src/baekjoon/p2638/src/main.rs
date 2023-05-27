use std::collections::HashSet;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dfs(field: &mut Vec<Vec<usize>>, i: usize, j: usize, n: usize, m: usize, visited: &mut Vec<Vec<bool>>, index: usize, island: &mut HashSet<usize>) {
    if field[i][j] != 0 || visited[i][j] {
        return;
    }

    visited[i][j] = true;
    
    if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
        island.insert(index);
    }
    field[i][j] = index;

    if i > 0 {
        dfs(field, i - 1, j, n, m, visited, index, island);
    }
    if i < n - 1 {
        dfs(field, i + 1, j, n, m, visited, index, island);
    }
    if j > 0 {
        dfs(field, i, j - 1, n, m, visited, index, island);
    }
    if j < m - 1 {
        dfs(field, i, j + 1, n, m, visited, index, island);
    }
}

fn get_island(field: &mut Vec<Vec<usize>>, n: usize, m: usize) -> Vec<usize> {
    let mut result = HashSet::new();
    let mut visited = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if field[i][j] == 0 && !visited[i][j] {
                dfs(field, i, j, n, m, &mut visited, result.len() + 2, &mut result);
            }
        }
    }
    result.into_iter().collect()
}

fn simulate(field: &mut Vec<Vec<usize>>, n: usize, m: usize) -> bool {
    let mut field_clone = field.clone();
    let island = get_island(&mut field_clone, n, m);
    let mut changed = false;
    for i in 0..n {
        for j in 0..m {
            if field[i][j] == 1 {
                let mut count = 0;
                if i > 0 {
                    if island.contains(&field_clone[i - 1][j]) {
                        count += 1;
                    }
                }
                if i < n - 1 {
                    if island.contains(&field_clone[i + 1][j]) {
                        count += 1;
                    }
                }
                if j > 0 {
                    if island.contains(&field_clone[i][j - 1]) {
                        count += 1;
                    }
                }
                if j < m - 1 {
                    if island.contains(&field_clone[i][j + 1]) {
                        count += 1;
                    }
                }
                if count >= 2 {
                    field[i][j] = 0;
                    changed = true;
                }
            }
        }
    }
    changed
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };
    let mut field = Vec::new();
    for _ in 0..n {
        let v = read_line_as_numbers();
        field.push(v);
    }

    let mut count = 0;
    while simulate(&mut field, n, m) {
        count += 1;
    }
    println!("{}", count);
}
