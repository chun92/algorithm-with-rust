fn read_line_as_numbers() -> Vec<i32> {
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
        let mut parent = Vec::with_capacity(n + 1);
        let mut rank = Vec::with_capacity(n + 1);

        for i in 0..=n {
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
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else {
            self.parent[py] = px;
            self.rank[px] += 1;
        }
    }
}

fn main() {
    let (v, e) = {
        let args = read_line_as_numbers();
        (args[0] as usize, args[1] as usize)
    };

    let mut uf = UnionFind::new(v);
    let mut edges = Vec::with_capacity(e);

    for _ in 0..e {
        let args = read_line_as_numbers();
        edges.push((args[0], args[1], args[2]));
    }

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut total = 0;
    for (u, v, w) in edges {
        let u = u as usize;
        let v = v as usize;
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            total += w;
        }
    }

    println!("{}", total);
}
