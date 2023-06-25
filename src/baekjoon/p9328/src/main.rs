
use std::collections::HashSet;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_map() -> (usize, usize, Vec<Vec<char>>) {
    let mut map = Vec::new();
    let (h, w) = {
        let line = read_line_as_numbers();
        (line[0], line[1])
    };
    for _ in 0..h {
        let line = read_line_as_string();
        map.push(line.chars().collect());
    }
    (h, w, map)
}

fn read_keys() -> HashSet<char> {
    let line = read_line_as_string();
    if line == "0" {
        return HashSet::new();
    }
    line.chars().collect()
}

fn check_one(map: &mut Vec<Vec<char>>, i: usize, j: usize, keys: &mut HashSet<char>, candidates: &mut Vec<(usize, usize)>) -> bool {
    match map[i][j] {
        '.' => true,
        '*' => false,
        '$' => true,
        'a'..='z' => {
            keys.insert(map[i][j]);
            map[i][j] = '.';
            return true;
        },
        'A'..='Z' => {
            let key = map[i][j].to_ascii_lowercase();
            if keys.contains(&key) {
                map[i][j] = '.';
                if candidates.contains(&((i, j))) {
                    candidates.retain(|&x| x != (i, j));
                }
                return true;
            } else {
                candidates.push((i, j));
                return false;
            }
        },
        _ => panic!("Invalid input {}", map[i][j]),
    }
}

fn check_boundary_one(map: &mut Vec<Vec<char>>, x: usize, y: usize, entries: &mut HashSet<(usize, usize)>, keys: &mut HashSet<char>) -> bool {
    match map[x][y] {
        '.' | '$' => {
            let item = entries.get(&((x, y)));
            if item.is_none() {
                entries.insert((x, y));
                return true;
            }
            return false;
        },
        'a'..='z' => {
            let item = entries.get(&((x, y)));
            keys.insert(map[x][y]);
            map[x][y] = '.';
            if item.is_none() {
                entries.insert((x, y));
                return true;
            }
            return false;
        },
        'A'..='Z' => {
            let item = entries.get(&((x, y)));
            let key = map[x][y].to_ascii_lowercase();
            if keys.contains(&key) {
                map[x][y] = '.';
                if item.is_none() {
                    entries.insert((x, y));
                    return true;
                }
            }
            return false;
        },
        '*' => false,
        _ => panic!("Invalid input {}", map[x][y]),
    }
}

fn check_boundary(map: &mut Vec<Vec<char>>, h: usize, w: usize, entries: &mut HashSet<(usize, usize)>, keys: &mut HashSet<char>) -> bool {
    let mut flag = false;
    for i in 0..h {
        if check_boundary_one(map, i, 0, entries, keys) {
            flag = true;
        }

        if check_boundary_one(map, i, w - 1, entries, keys) {
            flag = true;
        }
    }
    for j in 0..w {
        if check_boundary_one(map, 0, j, entries, keys) {
            flag = true;
        }

        if check_boundary_one(map, h - 1, j, entries, keys) {
            flag = true;
        }
    }
    flag
}

fn mark_map(map: &mut Vec<Vec<char>>, keys: &mut HashSet<char>, entries: &mut HashSet<(usize, usize)>, visited: &mut Vec<Vec<bool>>, candidates: &mut Vec<(usize, usize)>) {
    let h = map.len();
    let w = map[0].len();

    loop {
        check_boundary(map, h, w, entries, keys);
        let mut queue = Vec::new();
        for &(i, j) in entries.iter() {
            queue.push((i, j));
        }

        let mut possibles = Vec::new();
        for &(i, j) in candidates.iter() {
            match map[i][j] {
                'A'..='Z' => {
                    let key = map[i][j].to_ascii_lowercase();
                    if keys.contains(&key) {
                        map[i][j] = '.';
                        possibles.push((i, j));
                    }
                },
                _ => {}
            }
        }

        for &(i, j) in possibles.iter() {
            candidates.retain(|&x| x != (i, j));
            queue.push((i, j));
        }
        
        let mut changed = false;

        while let Some((x, y)) = queue.pop() {
            if visited[x][y] {
                continue;
            }

            visited[x][y] = true;
            changed = true;
            if x > 0 && check_one(map, x - 1, y, keys, candidates) {
                queue.push((x - 1, y));
            }

            if x < h - 1 && check_one(map, x + 1, y, keys, candidates) {
                queue.push((x + 1, y));
            }

            if y > 0 && check_one(map, x, y - 1, keys, candidates) {
                queue.push((x, y - 1));
            }

            if y < w - 1 && check_one(map, x, y + 1, keys, candidates) {
                queue.push((x, y + 1));
            }
        }

        if changed == false {
            break;
        }
    }
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let (h, w, mut map) = read_map();
        let mut keys = read_keys();

        let mut entries: HashSet<(usize, usize)> = HashSet::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
        let mut candidates: Vec<(usize, usize)> = Vec::new();

        mark_map(&mut map, &mut keys, &mut entries, &mut visited, &mut candidates);

        let mut sum = 0;
        for i in 0..h {
            for j in 0..w {
                if map[i][j] == '$' && visited[i][j] {
                    sum += 1;
                }
            }
        }
        println!("{}", sum);
    }
}
