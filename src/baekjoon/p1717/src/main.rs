use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find(x: usize, vec: &mut Vec<usize>) -> usize {
    if vec[x] == x {
        x
    } else {
        let path = find(vec[x], vec);
        vec[x] = path;
        path
    }
}

fn union(x: usize, y: usize, vec: &mut Vec<usize>, rank: &mut Vec<usize>) {
    let x = find(x, vec);
    let y = find(y, vec);
    if x != y {
        if rank[x] < rank[y] {
            vec[x] = y;
        } else {
            vec[y] = x;
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
        }
    }
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut out = BufWriter::new(stdout());
    let mut vec = (0..n + 1).collect::<Vec<usize>>();
    let mut rank = vec![0; n + 1];
    for _ in 0..m {
        let v = read_line_as_numbers();
        if v[0] == 0 {
            let (x, y) = (v[1], v[2]);
            union(x, y, &mut vec, &mut rank);
        } else {
            let (x, y) = (v[1], v[2]);
            let x = find(x, &mut vec);
            let y = find(y, &mut vec);
            if x == y {
                writeln!(out, "YES").unwrap();
            } else {
                writeln!(out, "NO").unwrap();
            }
        }
    }
}
