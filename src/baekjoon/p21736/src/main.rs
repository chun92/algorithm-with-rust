fn read_lien_as_numbers() -> Vec<usize> {
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

fn main() {
    let (n, m) = {
        let v = read_lien_as_numbers();
        (v[0], v[1])
    };

    let mut graph = vec![vec!['O'; m]; n];
    let mut start = (0, 0);
    for i in 0..n {
        let line = read_line_as_string();
        for (j, c) in line.chars().enumerate() {
            graph[i][j] = c;
            if c == 'I' {
                start = (i, j);
            }
        }
    }

    let mut stack = vec![start];
    let mut visited = vec![vec![false; m]; n];
    let mut count = 0;
    while let Some((i, j)) = stack.pop() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        if graph[i][j] == 'X' {
            continue;
        }
        if graph[i][j] == 'P' {
            count += 1;
        }
        if i > 0 {
            stack.push((i - 1, j));
        }
        if i < n - 1 {
            stack.push((i + 1, j));
        }
        if j > 0 {
            stack.push((i, j - 1));
        }
        if j < m - 1 {
            stack.push((i, j + 1));
        }
    }
    
    if count == 0 {
        println!("TT");
    } else {
        println!("{}", count);
    } 
}
