fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let (r, c, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut map = vec![vec![vec![]; c]; r];
    for _ in 0..m {
        let (r, c, s, d, z) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2], v[3], v[4])
        };
        map[r - 1][c - 1].push((s, d, z));
    }

    let mut ans = 0;
    for i in 0..c {
        for j in 0..r {
            if !map[j][i].is_empty() {
                ans += map[j][i][0].2;
                map[j][i].clear();
                break;
            }
        }

        let mut new_map = vec![vec![vec![]; c]; r];
        for i in 0..r {
            for j in 0..c {
                if !map[i][j].is_empty() {
                    assert!(map[i][j].len() == 1);
                    let (s, d, z) = map[i][j][0];
                    match d {
                        1 => {
                            if i >= s {
                                new_map[i - s][j].push((s, d, z));
                            } else {
                                let s1 = s - i;
                                let pos = s1 % (r - 1);
                                let dirrection = s1 / (r - 1);

                                if dirrection % 2 == 0 {
                                    new_map[pos][j].push((s, 2, z));
                                } else {
                                    new_map[r - 1 - pos][j].push((s, 1, z));
                                }
                            }
                        },
                        2 => {
                            if i + s < r {
                                new_map[i + s][j].push((s, d, z));
                            } else {
                                let s1 = s - (r - 1 - i);
                                let pos = s1 % (r - 1);
                                let dirrection = s1 / (r - 1);

                                if dirrection % 2 == 0 {
                                    new_map[r - 1 - pos][j].push((s, 1, z));
                                } else {
                                    new_map[pos][j].push((s, 2, z));
                                }
                            }
                        },
                        3 => {
                            if j + s < c {
                                new_map[i][j + s].push((s, d, z));
                            } else {
                                let s1 = s - (c - 1 - j);
                                let pos = s1 % (c - 1);
                                let dirrection = s1 / (c - 1);

                                if dirrection % 2 == 0 {
                                    new_map[i][c - 1 - pos].push((s, 4, z));
                                } else {
                                    new_map[i][pos].push((s, 3, z));
                                }
                            }
                        },
                        4 => {
                            if j >= s {
                                new_map[i][j - s].push((s, d, z));
                            } else {
                                let s1 = s - j;
                                let pos = s1 % (c - 1);
                                let dirrection = s1 / (c - 1);

                                if dirrection % 2 == 0 {
                                    new_map[i][pos].push((s, 3, z));
                                } else {
                                    new_map[i][c - 1 - pos].push((s, 4, z));
                                }
                            }
                        },
                        _ => panic!("invalid dirrection")
                    }
                }
            }
        }
        for i in 0..r {
            for j in 0..c {
                if new_map[i][j].len() > 1 {
                    new_map[i][j].sort_by(|a, b| b.2.cmp(&a.2));
                    let first = new_map[i][j][0];
                    new_map[i][j].clear();
                    new_map[i][j].push(first);
                }
            }
        }
        map = new_map;
    }

    println!("{}", ans);
}
