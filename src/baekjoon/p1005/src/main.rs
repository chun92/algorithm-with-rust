struct Edge {
    idx: usize,
    weight: usize,
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(vertices: &Vec<usize>, edges: Vec<Vec<usize>>, enters: &mut Vec<usize>, target: usize) -> usize {
    let mut sum = 0;
    let mut queue = Vec::new();
    for (i, &enter) in enters.iter().enumerate() {
        if enter == 0 {
            queue.push(Edge { idx: i, weight: vertices[i] });
        }
    }

    queue.sort_unstable_by(|a, b| b.weight.cmp(&a.weight));

    while let Some(Edge { idx, weight }) = queue.pop() {
        sum += weight;

        if idx == target {
            break;
        }

        for i in 0..queue.len() {
            queue[i].weight -= weight;
        }
        
        for &next in &edges[idx] {
            enters[next] -= 1;
            if enters[next] == 0 {
                queue.push(Edge { idx: next, weight: vertices[next] });
            }
        }

        queue.sort_unstable_by(|a, b| b.weight.cmp(&a.weight));
    }
    sum
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let (n, k) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        let vertices = read_line_as_numbers();
        let mut edges = vec![Vec::new(); n];
        let mut enters = vec![0; n];
        for _ in 0..k {
            let (u, v) = {
                let v = read_line_as_numbers();
                (v[0] - 1, v[1] - 1)
            };
            edges[u].push(v);
            enters[v] += 1;
        }
        let target = read_line_as_numbers()[0] - 1;

        let result = solve(&vertices, edges, &mut enters, target);
        println!("{}", result);
    }
}
