use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Dist {
    Inf,
    Num(usize),
}

fn main() {
    let (n, m) = (read_line_as_numbers()[0], read_line_as_numbers()[0]);
    let mut graph = vec![vec![Dist::Inf; n]; n];
    let mut next = vec![vec![Dist::Inf; n]; n];

    for i in 0..n {
        graph[i][i] = Dist::Num(0);
    }

    for _ in 0..m {
        let args = read_line_as_numbers();
        let (a, b, c) = (args[0], args[1], args[2]);
        if let Dist::Num(x) = graph[a - 1][b - 1] {
            graph[a - 1][b - 1] = Dist::Num(x.min(c));
        } else {
            graph[a - 1][b - 1] = Dist::Num(c);
        }
        next[a - 1][b - 1] = Dist::Num(b - 1);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                match (graph[i][k], graph[k][j]) {
                    (Dist::Num(x), Dist::Num(y)) => {
                        match graph[i][j] {
                            Dist::Inf => {
                                graph[i][j] = Dist::Num(x + y);
                                next[i][j] = next[i][k];
                            },
                            Dist::Num(z) => {
                                if z > x + y {
                                    graph[i][j] = Dist::Num(x + y);
                                    next[i][j] = next[i][k];
                                }
                            },
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut writer = BufWriter::new(stdout());
    for i in 0..n {
        for j in 0..n {
            match graph[i][j] {
                Dist::Inf => write!(writer, "0").unwrap(),
                Dist::Num(x) => write!(writer, "{}", x).unwrap(),
            }
            if j != n - 1 {
                write!(writer, " ").unwrap();
            }
        }
        writeln!(writer, "").unwrap();
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                writeln!(writer, "0").unwrap();
                continue;
            }
            let mut path = vec![i];
            let mut cur = i;
            let mut infinity = false;
            while cur != j {
                match next[cur][j] {
                    Dist::Inf => {
                        infinity = true;
                        break;
                    },
                    Dist::Num(x) => {
                        cur = x;
                        path.push(cur);
                    },
                }
            }
            if infinity {
                writeln!(writer, "0").unwrap();
                continue;
            }
            write!(writer, "{} ", path.len()).unwrap();
            
            let str =path
                .iter()
                .map(|x| (x + 1).to_string())
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(writer, "{}", str).unwrap();
        }
    }
}