use std::collections::VecDeque;
use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn find_possible_blocks(graph: &Vec<Vec<usize>>, n: usize, m: usize) {
    let mut blocks = vec![vec![(0, 0); m]; n];

    let mut queue = VecDeque::new();
    let mut visitied = vec![vec![false; m]; n];
    let mut number = 0;
    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == 0 && !visitied[i][j] {
                number += 1;
                let mut result = 0;
                let mut result_vec = Vec::new();
                queue.push_back((i, j));
                visitied[i][j] = true;
                while let Some((x, y)) = queue.pop_front() {
                    result += 1;
                    result_vec.push((x, y));
                    if x > 0 && graph[x - 1][y] == 0 && !visitied[x - 1][y] {
                        queue.push_back((x - 1, y));
                        visitied[x - 1][y] = true;
                    }
                    if x < n - 1 && graph[x + 1][y] == 0 && !visitied[x + 1][y] {
                        queue.push_back((x + 1, y));
                        visitied[x + 1][y] = true;
                    }
                    if y > 0 && graph[x][y - 1] == 0 && !visitied[x][y - 1] {
                        queue.push_back((x, y - 1));
                        visitied[x][y - 1] = true;
                    }
                    if y < m - 1 && graph[x][y + 1] == 0 && !visitied[x][y + 1] {
                        queue.push_back((x, y + 1));
                        visitied[x][y + 1] = true;
                    }
                }
                for (x, y) in result_vec {
                    blocks[x][y] = (number, result % 10);
                }
            }
        }
    }

    let mut result_vec = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == 1 {
                let mut result_set = std::collections::HashSet::new();
                if i > 0 && graph[i - 1][j] == 0 {
                    result_set.insert(blocks[i - 1][j]);
                }
                if i < n - 1 && graph[i + 1][j] == 0 {
                    result_set.insert(blocks[i + 1][j]);
                }
                if j > 0 && graph[i][j - 1] == 0 {
                    result_set.insert(blocks[i][j - 1]);
                }
                if j < m - 1 && graph[i][j + 1] == 0 {
                    result_set.insert(blocks[i][j + 1]);
                }
                let mut result = 1;
                for (_, count) in result_set {
                    result += count;
                }
                result_vec[i][j] = result % 10;
            }
        }
    }

    let mut output = BufWriter::new(stdout());
    result_vec
        .iter()
        .for_each(|v| {
            v.iter().for_each(|n| write!(output, "{}", n).unwrap());
            writeln!(output).unwrap();
        });
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = Vec::new();
    for _ in 0..n {
        let s = read_line_as_string();
        let vec = s
            .chars()
            .map(|c| {
                c.to_digit(10).unwrap() as usize
            })
            .collect::<Vec<usize>>();
        graph.push(vec);
    }

    find_possible_blocks(&graph, n, m);
}
