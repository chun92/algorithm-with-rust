use std::{io::{stdin, stdout, Write, BufWriter}, collections::VecDeque};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut graph = Vec::new();
    for _ in 0..n {
        graph.push(read_line_as_numbers());
    }

    let mut result = vec![vec![0; n]; n];

    for i in 0..n {
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        
        for j in 0..n {
            if graph[i][j] == 1 {
                queue.push_back(j);
            }
        }

        while let Some(j) = queue.pop_front() {
            if visited[j] {
                continue;
            }
            visited[j] = true;
            result[i][j] = 1;
            for k in 0..n {
                if graph[j][k] == 1 {
                    queue.push_back(k);
                }
            }
        }
    }

    let mut output = BufWriter::new(stdout());
    for i in 0..n {
        for j in 0..n {
            if j > 0 {
                write!(output, " ").unwrap();
            }
            write!(output, "{}", result[i][j]).unwrap();
        }
        writeln!(output).unwrap();
    }
}
