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
    let (n, m) = {
        let args = read_line_as_numbers();
        (args[0], args[1])
    };

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let args = read_line_as_numbers();
        points.push((args[0] as f64, args[1] as f64));
    }

    let mut uf = UnionFind::new(n);
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let w = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
            edges.push((i, j, w));
        }
    }

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for _ in 0..m {
        let args = read_line_as_numbers();
        let u = args[0] - 1;
        let v = args[1] - 1;
        uf.union(u, v);
    }

    let mut total = 0.0;
    for (u, v, w) in edges {
        let u = u as usize;
        let v = v as usize;
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            total += w;
        }
    }

    println!("{:.2}", total);
}
