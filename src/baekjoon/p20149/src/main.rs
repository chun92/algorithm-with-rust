use std::io::{self, BufRead};
use std::str::SplitWhitespace;

struct Point {
    x: i128,
    y: i128,
}

fn read_numbers(line: &mut SplitWhitespace) -> (i128, i128) {
    let x: i128 = line.next().unwrap().parse().unwrap();
    let y: i128 = line.next().unwrap().parse().unwrap();
    (x, y)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i128 {
    let val = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if val > 0 {
        1
    } else if val < 0 {
        -1
    } else {
        0
    }
}

fn is_intersect(a: &Point, b: &Point, c: &Point, d: &Point) -> usize {
    let ac = ccw(a, b, c);
    let ad = ccw(a, b, d);
    let ca = ccw(c, d, a);
    let cb = ccw(c, d, b);
    let ab = ac * ad;
    let cd = ca * cb;
    if ab == 0 && cd == 0 {
        if !(ac == 0 && ad == 0 && ca == 0 && cb == 0) {
            return 3
        }

        if (std::cmp::min(a.x, b.x) == std::cmp::max(c.x, d.x)
            && std::cmp::min(a.y, b.y) == std::cmp::max(c.y, d.y))
            || (std::cmp::min(c.x, d.x) == std::cmp::max(a.x, b.x)
            && std::cmp::min(c.y, d.y) == std::cmp::max(a.y, b.y))
            || (std::cmp::min(a.x, b.x) == std::cmp::max(c.x, d.x)
            && std::cmp::min(c.y, d.y) == std::cmp::max(a.y, b.y))
            || (std::cmp::min(c.x, d.x) == std::cmp::max(a.x, b.x)
            && std::cmp::min(a.y, b.y) == std::cmp::max(c.y, d.y)) {
            return 3
        }

        let result = std::cmp::min(a.x, b.x) <= std::cmp::max(c.x, d.x)
            && std::cmp::min(a.y, b.y) <= std::cmp::max(c.y, d.y)
            && std::cmp::min(c.x, d.x) <= std::cmp::max(a.x, b.x)
            && std::cmp::min(c.y, d.y) <= std::cmp::max(a.y, b.y);
        if result {
            return 2
        } else {
            return 0
        }
    }
    let result = ab <= 0 && cd <= 0;
    if result {
        return 1
    } else {
        return 0
    }
}

enum FirstOrderEquation {
    Linear(f64, f64),
    Vertical(f64),
    Horizontal(f64),
}

enum Intersection {
    None,
    One(f64, f64),
    Infinite,
}

impl FirstOrderEquation {
    fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        if x1 == x2 {
            FirstOrderEquation::Vertical(x1)
        } else if y1 == y2 {
            FirstOrderEquation::Horizontal(y1)
        } else {
            let a = (y2 - y1) / (x2 - x1);
            let b = y1 - a * x1;
            FirstOrderEquation::Linear(a, b)
        }
    }

    fn check_intersection(&self, other: &Self) -> Intersection {
        match (self, other) {
            (FirstOrderEquation::Vertical(x1), FirstOrderEquation::Vertical(x2)) => {
                if x1 == x2 {
                    Intersection::Infinite
                } else {
                    Intersection::None
                }
            },
            (FirstOrderEquation::Horizontal(y1), FirstOrderEquation::Horizontal(y2)) => {
                if y1 == y2 {
                    Intersection::Infinite
                } else {
                    Intersection::None
                }
            },
            (FirstOrderEquation::Vertical(x1), FirstOrderEquation::Horizontal(y2)) => {
                Intersection::One(*x1, *y2)
            },
            (FirstOrderEquation::Horizontal(y1), FirstOrderEquation::Vertical(x2)) => {
                Intersection::One(*x2, *y1)
            },
            (FirstOrderEquation::Linear(a1, b1), FirstOrderEquation::Linear(a2, b2)) => {
                if a1 == a2 {
                    if b1 == b2 {
                        Intersection::Infinite
                    } else {
                        Intersection::None
                    }
                } else {
                    let x = (b2 - b1) / (a1 - a2);
                    let y = a1 * x + b1;
                    Intersection::One(x, y)
                }
            },
            (FirstOrderEquation::Linear(a1, b1), FirstOrderEquation::Vertical(x2)) => {
                let y = a1 * x2 + b1;
                Intersection::One(*x2, y)
            },
            (FirstOrderEquation::Vertical(x1), FirstOrderEquation::Linear(a2, b2)) => {
                let y = a2 * x1 + b2;
                Intersection::One(*x1, y)
            },
            (FirstOrderEquation::Linear(a1, b1), FirstOrderEquation::Horizontal(y2)) => {
                let x = (y2 - b1) / a1;
                Intersection::One(x, *y2)
            },
            (FirstOrderEquation::Horizontal(y1), FirstOrderEquation::Linear(a2, b2)) => {
                let x = (y1 - b2) / a2;
                Intersection::One(x, *y1)
            },
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut l1_line = line.split_whitespace();
    let (x1, y1) = read_numbers(&mut l1_line);
    let (x2, y2) = read_numbers(&mut l1_line);

    let line = lines.next().unwrap().unwrap();
    let mut l2_line = line.split_whitespace();
    let (x3, y3) = read_numbers(&mut l2_line);
    let (x4, y4) = read_numbers(&mut l2_line);

    let p1 = Point { x: x1, y: y1 };
    let p2 = Point { x: x2, y: y2 };
    let p3 = Point { x: x3, y: y3 };
    let p4 = Point { x: x4, y: y4 };

    let result = is_intersect(&p1, &p2, &p3, &p4);
    match result {
        0 => println!("0"),
        1 => {
            println!("1");
            let p1 = (p1.x as f64, p1.y as f64);
            let p2 = (p2.x as f64, p2.y as f64);
            let p3 = (p3.x as f64, p3.y as f64);
            let p4 = (p4.x as f64, p4.y as f64);
            let e1 = FirstOrderEquation::new(p1, p2);
            let e2 = FirstOrderEquation::new(p3, p4);
            let intersection = e1.check_intersection(&e2);

            match intersection {
                Intersection::One(x, y) => {
                    println!("{} {}", x, y);
                },
                _ => unreachable!(),
            }
        },
        2 => println!("1"),
        3 => {
            println!("1");
            if p1.x == p3.x && p1.y == p3.y {
                println!("{} {}", p1.x, p1.y);
            } else if p1.x == p4.x && p1.y == p4.y {
                println!("{} {}", p1.x, p1.y);
            } else if p2.x == p3.x && p2.y == p3.y {
                println!("{} {}", p2.x, p2.y);
            } else if p2.x == p4.x && p2.y == p4.y {
                println!("{} {}", p2.x, p2.y);
            } else {
                unreachable!();
            }
        }
        _ => unreachable!(),
    }
}
