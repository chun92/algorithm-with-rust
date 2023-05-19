fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dfs(i: usize, j: usize, number: usize, visited: &mut Vec<Vec<bool>>, world: &mut Vec<Vec<usize>>) {
    visited[i][j] = true;
    world[i][j] = number;
    let n = world.len();
    let m = world[0].len();

    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, 1, 0, -1];

    for k in 0..4 {
        let x = i as i32 + dx[k];
        let y = j as i32 + dy[k];
        if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
            let x = x as usize;
            let y = y as usize;
            if world[x][y] == 1 && !visited[x][y] {
                world[x][y] = number;
                dfs(x, y, number, visited, world);
            }
        }
    }
}

fn make_island() -> (Vec<Vec<usize>>, usize) {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut world = vec![vec![0; m]; n];

    for i in 0..n {
        let line = read_line_as_numbers();
        for j in 0..m {
            world[i][j] = line[j];
        }
    }

    let mut visited = vec![vec![false; m]; n];
    let mut number = 1;
    for i in 0..n {
        for j in 0..m {
            if world[i][j] == 1 && !visited[i][j] {
                dfs(i, j, number, &mut visited, &mut world);
                number += 1;
            }
        }
    }

    (world, number)
}

fn make_edges(world: &Vec<Vec<usize>>) -> Vec<(usize, usize, usize)> {
    let n = world.len();
    let m = world[0].len();

    let mut edges = Vec::new();

    for i in 0..n {
        let mut current = 0;
        let mut count = 0;
        for j in 0..m {
            let value = world[i][j];
            if value == 0 {
                if current != 0 {
                    count += 1;
                }
            } else {
                if current == 0 {
                    current = value;
                } else {
                    if count > 1 && current != value {
                        edges.push((current, value, count));
                    }
                    current = value;
                    count = 0;
                }
            }
        }
    }

    for j in 0..m {
        let mut current = 0;
        let mut count = 0;
        for i in 0..n {
            let value = world[i][j];
            if value == 0 {
                if current != 0 {
                    count += 1;
                }
            } else {
                if current == 0 {
                    current = value;
                } else {
                    if count > 1 && current != value {
                        edges.push((current, value, count));
                    }
                    current = value;
                    count = 0;
                }
            }
        }
    }

    edges
}

fn _print_world(world: &Vec<Vec<usize>>) {
    for i in 0..world.len() {
        for j in 0..world[0].len() {
            print!("{} ", world[i][j]);
        }
        println!();
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parent = vec![0; n];
        let rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind {
            parent: parent,
            rank: rank,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

fn make_mst(edges: &Vec<(usize, usize, usize)>, number: usize) -> (usize, bool) {
    let mut parent = vec![0; number];
    for i in 0..number {
        parent[i] = i;
    }

    let mut edges = edges.clone();
    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(number);
    let mut sum = 0;

    for edge in edges {
        let (a, b, w) = edge;
        if uf.find(a) != uf.find(b) {
            uf.union(a, b);
            sum += w;
        }
    }

    let mut count = 0;
    for i in 1..number {
        if uf.find(i) == i {
            count += 1;
        }
    }

    (sum, count == 1)
}

fn solve(world: &Vec<Vec<usize>>, number: usize) -> Option<usize> {
    let edges = make_edges(world);
    let (sum, is_one) = make_mst(&edges, number);
    if is_one {
        Some(sum)
    } else {
        None
    }
}

fn main() {
    let (world, number) = make_island();
    let answer = solve(&world, number);
    match answer {
        Some(answer) => println!("{}", answer),
        None => println!("-1"),
    }
}

