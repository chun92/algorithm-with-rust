use std::{collections::BinaryHeap, cmp::Reverse};
use std::io::{self, BufRead};

fn read_line_as_numbers() -> Vec<i32> {
    io::stdin().lock().lines().next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut rank = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            rank.push(0);
        }
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }

        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else {
            self.parent[py] = px;
            if self.rank[px] == self.rank[py] {
                self.rank[px] += 1;
            }
        }
    }
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let mut vec = Vec::new();
    for i in 0..n {
        let (x, y, z) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2])
        };
        vec.push((i, x, y, z));
    }

    let mut heap = BinaryHeap::new();
    vec.sort_unstable_by_key(|(i, x, y, z)| *x);
    for i in 0..n-1 {
        let (i1, x1, y1, z1) = vec[i];
        let (i2, x2, y2, z2) = vec[i+1];
        heap.push(Reverse((x2-x1, i1, i2)));
    }

    vec.sort_unstable_by_key(|(i, x, y, z)| *y);
    for i in 0..n-1 {
        let (i1, x1, y1, z1) = vec[i];
        let (i2, x2, y2, z2) = vec[i+1];
        heap.push(Reverse((y2-y1, i1, i2)));
    }

    vec.sort_unstable_by_key(|(i, x, y, z)| *z);
    for i in 0..n-1 {
        let (i1, x1, y1, z1) = vec[i];
        let (i2, x2, y2, z2) = vec[i+1];
        heap.push(Reverse((z2-z1, i1, i2)));
    }
    
    let mut ans = 0;

    let mut uf = UnionFind::new(n);
    while let Some(Reverse((cost, i1, i2))) = heap.pop() {
        if uf.find(i1) != uf.find(i2) {
            uf.union(i1, i2);
            ans += cost;
        }
    }

    println!("{}", ans);
}
