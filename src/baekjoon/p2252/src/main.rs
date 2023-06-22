fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = vec![vec![]; n + 1];
    let mut indegree = vec![0; n + 1];
    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        graph[a].push(b);
        indegree[b] += 1;
    }

    let mut vec = Vec::new();
    for i in 1..=n {
        if indegree[i] == 0 {
            vec.push(i);
        }
    }

    while !vec.is_empty() {
        let v = vec.pop().unwrap();
        print!("{} ", v);
        for i in &graph[v] {
            indegree[*i] -= 1;
            if indegree[*i] == 0 {
                vec.push(*i);
            }
        }
    }
}
