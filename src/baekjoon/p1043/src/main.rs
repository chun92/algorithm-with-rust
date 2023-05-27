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
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut known = vec![false; n];
    let args = read_line_as_numbers();

    for i in 1..=args[0] {
        known[args[i] - 1] = true;
    }

    let mut parties = Vec::new();

    for _ in 0..m {
        let args = read_line_as_numbers();
        let mut party = Vec::new();
        for i in 0..args[0] {
            party.push(args[i + 1] - 1);
        }
        parties.push(party);
    }

    let mut uf = UnionFind::new(n);

    for i in 0..m {
        let party = &parties[i];
        for j in 0..party.len() {
            uf.union(party[0], party[j]);
        }
    }

    for i in 0..n {
        let p = uf.find(i);
        if known[i] {
            known[p] = true;
        }
    }

    let mut ans = 0;
    for i in 0..m {
        let party = &parties[i];
        let mut flag = true;
        for j in 0..party.len() {
            if known[uf.find(party[j])] {
                flag = false;
                break;
            }
        }
        if flag {
            ans += 1;
        }
    }

    println!("{}", ans);
}
