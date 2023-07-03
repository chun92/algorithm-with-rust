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
            let parent = self.parent[x];
            let root = self.find(parent);
            self.parent[x] = root;
            root
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

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

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m, k) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let vec = read_line_as_numbers();
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1)
        };
        uf.union(a, b);
    }

    let mut friends = vec![0; n];
    let mut values = vec![0; n];
    for i in 0..n {
        let root = uf.find(i);
        friends[root] += 1;
        values[root] += vec[i];
    }

    let mut memo = vec![0; k];
    for i in 0..n {
        if friends[i] == 0 {
            continue;
        }
        let cur_friends = friends[i];
        let cur_value = values[i];
        for j in (cur_friends..k).rev() {
            memo[j] = memo[j].max(memo[j - cur_friends] + cur_value);
        }
    }

    println!("{}", memo[k - 1]);
}
