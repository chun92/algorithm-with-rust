fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn contains(&self, x: i32, y: i32) -> bool {
        if self.w == 0 && self.h == 0 {
            return self.x == x && self.y == y;
        }

        x > self.x - self.w && x <= self.x + self.w &&
        y > self.y - self.h && y <= self.y + self.h
    }
}

#[derive(Debug)]
struct QuadTree<T: PartialEq + Copy> {
    boundary: Rect,
    data: Option<T>,
    children: [Option<Box<QuadTree<T>>>; 4],
}

impl<T: PartialEq + Copy> QuadTree<T> {
    fn new(n: i32) -> QuadTree<T> {
        QuadTree {
            boundary: Rect { x: n / 2, y: n / 2, w: n / 2, h: n / 2 },
            data: None,
            children: [None, None, None, None],
        }
    }

    fn subdivide(&mut self) {
        if self.boundary.w == 0 || self.boundary.h  == 0 {
            return;
        }

        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w / 2;
        let h = self.boundary.h / 2;

        fn make_subtree<T: PartialEq + Copy> (x: i32, y: i32, w: i32, h: i32) -> QuadTree<T> {
            QuadTree {
                boundary: Rect { x: x, y: y, w: w, h: h },
                data: None,
                children: [None, None, None, None],
            }
        }

        if w == 0 || h == 0 {
            self.children[0] = Some(Box::new(make_subtree(x, y, w, h)));
            self.children[1] = Some(Box::new(make_subtree(x + 1, y, w, h)));
            self.children[2] = Some(Box::new(make_subtree(x, y + 1, w, h)));
            self.children[3] = Some(Box::new(make_subtree(x + 1, y + 1, w, h)));
        } else {
            self.children[0] = Some(Box::new(make_subtree(x - w, y - h, w, h)));
            self.children[1] = Some(Box::new(make_subtree(x + w, y - h, w, h)));
            self.children[2] = Some(Box::new(make_subtree(x - w, y + h, w, h)));
            self.children[3] = Some(Box::new(make_subtree(x + w, y + h, w, h)));
        }
    }

    fn check_and_merge(&mut self) {
        let mut data = None;
        for child in self.children.iter() {
            if let Some(ref c) = *child {
                if c.data.is_some() {
                    if data.is_some() {
                        if data != c.data {
                            return;
                        }
                    } else {
                        data = Some(*c.data.as_ref().unwrap());
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        self.data = data;
        self.children = [None, None, None, None];
    }

    fn insert(&mut self, x: i32, y: i32, data: T) {
        if !self.boundary.contains(x, y) {
            return;
        }

        if self.data.is_some() {
            if self.data == Some(data) {
                return;
            } else {
                self.subdivide();
            }
        } else {
            if self.children[0].is_none() {
                self.subdivide();
            }
        }

        if self.children[0].is_some() {
            for child in self.children.iter_mut() {
                child.as_mut().unwrap().insert(x, y, data);
            }
            self.check_and_merge();
        } else {
            self.data = Some(data);
        }
    }

    fn size_of_values(&self, target: T) -> (usize, usize) {
        if self.data.is_some() {
            if self.data.unwrap() == target {
                (1, 0)
            } else {
                (0, 1)
            }
        } else {
            self.children.iter().fold((0, 0), |acc, c| {
                let (a, b) = c.as_ref().unwrap().size_of_values(target);
                (acc.0 + a, acc.1 + b)
            })
        }
    }
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut tree = QuadTree::<bool>::new(n);
    for i in 1..=n {
        let data = read_line_as_numbers();
        for (j, d) in data.iter().enumerate() {
            let j = j + 1;
            if *d == 1 {
                tree.insert(j as i32, i, true);
            } else if *d == 0 {
                tree.insert(j as i32, i, false);
            }
        }
    }
    let (a, b) = tree.size_of_values(false);
    println!("{}", a);
    println!("{}", b);
}
