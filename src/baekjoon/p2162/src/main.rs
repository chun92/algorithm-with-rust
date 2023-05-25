struct Point {
    x: i64,
    y: i64,
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let val = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if val > 0 {
        1
    } else if val < 0 {
        -1
    } else {
        0
    }
}

fn is_intersect(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    let ab = ccw(a, b, c) * ccw(a, b, d);
    let cd = ccw(c, d, a) * ccw(c, d, b);
    if ab == 0 && cd == 0 {
        return std::cmp::min(a.x, b.x) <= std::cmp::max(c.x, d.x)
            && std::cmp::min(c.x, d.x) <= std::cmp::max(a.x, b.x)
            && std::cmp::min(a.y, b.y) <= std::cmp::max(c.y, d.y)
            && std::cmp::min(c.y, d.y) <= std::cmp::max(a.y, b.y);
    }
    ab <= 0 && cd <= 0
}

fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            self.parent[x] = self.root(p);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let rx = self.root(x);
        self.size[rx]
    }
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;

    let mut uf = UnionFind::new(n);
    let mut points = Vec::new();

    for i in 0..n {
        let (x1, y1, x2, y2) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2], v[3])
        };
        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };
        for j in 0..i {
            let (p3, p4): &(Point, Point) = points.get(j).unwrap();
            if is_intersect(&p1, &p2, p3, p4) {
                uf.unite(i, j);
            }
        }
        points.push((p1, p2));
    }

    let mut count = Vec::new();
    let mut max = 0;

    for i in 0..n {
        let root = uf.root(i);
        if count.contains(&root) {
            continue;
        }
        count.push(root);
        let size = uf.size(root);
        if size > max {
            max = size;
        }
    }
    println!("{}", count.len());
    println!("{}", max);
}
