use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Node {
    cost: usize,
    position: (usize, usize),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind {
            parent: parent,
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

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut uf = UnionFind::new(n);

    let mut edges = BinaryHeap::new();
    for _ in 0..m {
        let (a, b, c) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1, v[2])
        };
        edges.push(Node { cost: c, position: (a, b) });
    }

    let mut sum = 0;
    let mut max = 0;
    let mut count = 0;

    while let Some(edge) = edges.pop() {
        if !uf.same(edge.position.0, edge.position.1) {
            uf.unite(edge.position.0, edge.position.1);
            sum += edge.cost;
            max = std::cmp::max(max, edge.cost);
            count += 1;
        }

        if count == n - 1 {
            break;
        }
    }

    println!("{}", sum - max);
}
