fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

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
    loop {
        let (m, n) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };

        if m == 0 && n == 0 {
            break;
        }

        let mut uf = UnionFind::new(m);
        let mut edges = Vec::new();
        let mut total = 0;
        for _ in 0..n {
            let v = read_line_as_numbers();
            edges.push((v[0], v[1], v[2]));
            total += v[2];
        }

        let mut ans = 0;

        edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

        for (u, v, w) in edges {
            if uf.find(u) == uf.find(v) {
                continue;
            }
            uf.union(u, v);
            ans += w;
        }

        println!("{}", total - ans);
    }
}
