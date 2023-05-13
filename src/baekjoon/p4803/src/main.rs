use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut case = 1;
    let mut output = BufWriter::new(stdout());
    loop {
        let (n, m) = {
            let numbers = read_line_as_numbers();
            (numbers[0], numbers[1])
        };

        if n == 0 && m == 0 {
            break;
        }

        let mut graph = vec![vec![false; n + 1]; n + 1];
        for _ in 0..m {
            let (u, v) = {
                let numbers = read_line_as_numbers();
                (numbers[0], numbers[1])
            };
            graph[u][v] = true;
            graph[v][u] = true;
        }

        let mut count = 0;
        let mut visited = vec![false; n + 1];

        for i in 1..=n {
            if !visited[i] {
                let mut flag = true;
                let mut stack = vec![(i, i)];
                while let Some((u, prev)) = stack.pop() {
                    if !visited[u] {
                        visited[u] = true;
                        for v in 1..=n {
                            if v != prev && graph[u][v] {
                                stack.push((v, u));
                            }
                        }
                    } else {
                        flag = false;
                    }
                }
                if flag {
                    count += 1;
                }
            }
        }

        write!(output, "Case {}: ", case).unwrap();
        
        match count {
            0 => writeln!(output, "No trees.").unwrap(),
            1 => writeln!(output, "There is one tree.").unwrap(),
            _ => writeln!(output, "A forest of {} trees.", count).unwrap(),
        }

        case += 1;
    }
}
