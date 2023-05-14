struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root != y_root {
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
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let m = read_line_as_numbers()[0];

    let mut union_find = UnionFind::new(n);
    for i in 0..n {
        let numbers = read_line_as_numbers();
        for j in 0..n {
            if numbers[j] == 1 {
                union_find.union(i, j);
            }
        }
    }

    let numbers = read_line_as_numbers();
    let mut result = true;
    let root = union_find.find(numbers[0] - 1);
    for i in 1..m {
        if root != union_find.find(numbers[i] - 1) {
            result = false;
            break;
        }
    }

    if result {
        println!("YES");
    } else {
        println!("NO");
    }
}