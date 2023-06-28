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
        } else if self.rank[px] == self.rank[py] {
            self.parent[px] = py;
            self.rank[px] += 1;
        } else {
            self.parent[py] = px;
        }
    }
}

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let g = read_line_as_number();
    let p = read_line_as_number();

    let mut uf = UnionFind::new(g + 1);
    let mut count = 0;
    for _ in 0..p {
        let gi = read_line_as_number();
        let pi = uf.find(gi);
        if pi == 0 {
            break;
        }
        uf.unite(pi, pi - 1);
        count += 1;
    }

    println!("{}", count);
}
