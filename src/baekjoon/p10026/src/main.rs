fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn read_line_as_char_vector() -> Vec<char> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().chars().collect()
}


fn dfs(vec: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, current: char, is_r_equal_g: bool) {
    if visited[i][j] {
        return;
    }

    if is_r_equal_g {
        match (current, vec[i][j]) {
            ('R', 'G') => (),
            ('G', 'R') => (),
            (a, b) if a == b => (),
            _ => return,
        }
    } else if vec[i][j] != current {
        return;
    }

    visited[i][j] = true;

    if i > 0 {
        dfs(vec, visited, i - 1, j, current, is_r_equal_g);
    }
    if i < vec.len() - 1 {
        dfs(vec, visited, i + 1, j, current, is_r_equal_g);
    }
    if j > 0 {
        dfs(vec, visited, i, j - 1, current, is_r_equal_g);
    }
    if j < vec.len() - 1 {
        dfs(vec, visited, i, j + 1, current, is_r_equal_g);
    }
}
fn main() {
    let n = read_line_as_number();

    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_char_vector());
    }

    let mut visited = vec![vec![false; n]; n];

    let mut current ;
    let mut result1 = 0;

    for i in 0..n {
        for j in 0..n {
            if visited[i][j] {
                continue;
            }

            current = vec[i][j];
            dfs(&vec, &mut visited, i, j, current, false);
            result1 += 1;
        }
    }
    
    let mut visited = vec![vec![false; n]; n];
    let mut result2 = 0;
    for i in 0..n {
        for j in 0..n {
            if visited[i][j] {
                continue;
            }

            current = vec[i][j];
            dfs(&vec, &mut visited, i, j, current, true);
            result2 += 1;
        }
    }

    println!("{} {}", result1, result2);
}
