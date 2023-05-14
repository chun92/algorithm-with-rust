struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root != y_root {
            if self.rank[x_root] < self.rank[y_root] {
                self.parent[x_root] = y_root;
            } else if self.rank[x_root] > self.rank[y_root] {
                self.parent[y_root] = x_root;
            } else {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
    }
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let n = numbers[0];
    let m = numbers[1];

    let mut uf = UnionFind::new(n);

    let mut result = 0;
    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        if uf.find(numbers[0]) == uf.find(numbers[1]) {
            result = i + 1;
            break;
        }
        uf.union(numbers[0], numbers[1]);
    }

    println!("{}", result);
}