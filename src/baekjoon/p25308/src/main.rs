use std::collections::HashSet;

fn ccw(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    (x2 - x1) * (y3 - y2) - (x3 - x2) * (y2 - y1)
}

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let numbers = read_line_as_numbers();

    let root2 = 2.0f64.sqrt() / 2.0;

    let octagon = vec![
        (0.0, 1.0),
        (root2, root2),
        (1.0, 0.0),
        (root2, -root2),
        (0.0, -1.0),
        (-root2, -root2),
        (-1.0, 0.0),
        (-root2, root2),
    ];

    let mut count = 0;
    let set1 = (0..8).collect::<HashSet<usize>>();
    for a1 in set1.iter() {
        let mut set2 = set1.clone();
        set2.remove(a1);
        for a2 in set2.iter() {
            let mut set3 = set2.clone();
            set3.remove(a2);
            for a3 in set3.iter() {
                let mut set4 = set3.clone();
                set4.remove(a3);
                for a4 in set4.iter() {
                    let mut set5 = set4.clone();
                    set5.remove(a4);
                    for a5 in set5.iter() {
                        let mut set6 = set5.clone();
                        set6.remove(a5);
                            for a6 in set6.iter() {
                                let mut set7 = set6.clone();
                                set7.remove(a6);
                                for a7 in set7.iter() {
                                    let mut set8 = set7.clone();
                                    set8.remove(a7);
                                    for a8 in set8.iter() {
                                        let vec = vec![*a1, *a2, *a3, *a4, *a5, *a6, *a7, *a8];

                                        let mut result = true;
                                        for i in 0..8 {
                                            let j = (i + 1) % 8;
                                            let k = (i + 2) % 8;
                                            let point1 = (octagon[i].0 * numbers[vec[i]] as f64, octagon[i].1 * numbers[vec[i]] as f64);
                                            let point2 = (octagon[j].0 * numbers[vec[j]] as f64, octagon[j].1 * numbers[vec[j]] as f64);
                                            let point3 = (octagon[k].0 * numbers[vec[k]] as f64, octagon[k].1 * numbers[vec[k]] as f64);

                                            if ccw(point1, point2, point3) > 0.0 {
                                                result = false;
                                                break;
                                            }                                        
                                        }
                                        if result {
                                            count += 1;
                                        }
                                    }
                                    set8.insert(*a7);
                                }
                                set7.insert(*a6);
                            }
                        set6.insert(*a5);
                    }
                    set5.insert(*a4);
                }
                set4.insert(*a3);
            }
            set3.insert(*a2);
        }
        set2.insert(*a1);
    }

    println!("{}", count);
}