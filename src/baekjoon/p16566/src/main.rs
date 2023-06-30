use std::io::{stdin, stdout, Write, BufWriter};

struct UnionFind {
    parent: Vec<usize>,
    used: Vec<bool>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            used: vec![false; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.parent[x];
        if !self.used[parent] {
            return parent;
        }

        if self.parent[x] == x {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        self.parent[px] = py;
    }

    fn visit(&mut self, x: usize) {
        self.used[x] = true;
    }
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Failed to read line");
    line.split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn main() {
    let (n, _m, _k) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut targets = read_line_as_numbers();
    targets.sort_unstable();
    let mut uf = UnionFind::new(n + 1);

    let mut cur_index = 0;
    let mut cur_target = targets[cur_index];
    for i in 0..n + 1 {
        if i == cur_target {
            cur_index += 1;
            if cur_index < targets.len() {
                cur_target = targets[cur_index];
            }
        }
        uf.unite(i, cur_target);
    }

    let check = read_line_as_numbers();

    let mut output = BufWriter::new(stdout());

    for x in check {
        let px = uf.find(x);
        uf.visit(px);
        writeln!(output, "{}", px).unwrap();
    }
}
