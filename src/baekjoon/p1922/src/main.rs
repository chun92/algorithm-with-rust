use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
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

    fn unite(&mut self, x: usize, y: usize) {
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

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let m = read_line_as_numbers()[0];

    let mut heap = BinaryHeap::new();

    for _ in 0..m {
        let (a, b, w) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2])
        };
        heap.push(Reverse((w, a, b)));
    }

    let mut uf = UnionFind::new(n + 1);

    let mut ans = 0;

    while let Some(Reverse((w, a, b))) = heap.pop() {
        if uf.find(a) != uf.find(b) {
            uf.unite(a, b);
            ans += w;
        }
    }

    println!("{}", ans);
}
