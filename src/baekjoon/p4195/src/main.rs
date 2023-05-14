use std::collections::HashMap;

struct UnionFind {
    parent: HashMap<String, String>,
    size: HashMap<String, usize>,
}

impl UnionFind {
    fn new() -> UnionFind {
        UnionFind {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }

    fn make_set(&mut self, x: String) {
        self.parent.insert(x.clone(), x.clone());
        self.size.insert(x, 1);
    }

    fn find(&mut self, x: String) -> String {
        if self.parent[&x] != x {
            let parent = self.parent[&x].clone();
            let grandparent = self.find(parent.clone());
            self.parent.insert(x.clone(), grandparent);
        }
        self.parent[&x].clone()
    }
    
    fn union(&mut self, x: String, y: String) {
        let x_root = self.find(x.clone());
        let y_root = self.find(y.clone());

        if x_root != y_root {
            if self.size[&x_root] < self.size[&y_root] {
                self.parent.insert(x_root.clone(), y_root.clone());
                *self.size.get_mut(&y_root.clone()).unwrap() += self.size[&x_root.clone()];
            } else {
                self.parent.insert(y_root.clone(), x_root.clone());
                *self.size.get_mut(&x_root.clone()).unwrap() += self.size[&y_root.clone()];
            }
        }
    }

    fn get_size(&mut self, x: String) -> usize {
        let x_root = self.find(x);
        self.size[&x_root]
    }
}


use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut out = io::BufWriter::new(io::stdout());

    for _ in 0..n {
        let m = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut uf = UnionFind::new();
        for _ in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut words = line.split_whitespace();
            let x = words.next().unwrap().to_string();
            let y = words.next().unwrap().to_string();
            if !uf.parent.contains_key(&x) {
                uf.make_set(x.clone());
            }
            if !uf.parent.contains_key(&y) {
                uf.make_set(y.clone());
            }
            uf.union(x.clone(), y.clone());
            writeln!(out, "{}", uf.get_size(x)).unwrap();
        }
    }
}