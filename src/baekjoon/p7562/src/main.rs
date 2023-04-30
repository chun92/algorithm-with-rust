use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_knight(size: usize, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    let (x, y) = point;
    if x >= 2 && y >= 1 {
        v.push((x - 2, y - 1));
    }
    if x >= 2 && y < size - 1 {
        v.push((x - 2, y + 1));
    }
    if x >= 1 && y >= 2 {
        v.push((x - 1, y - 2));
    }
    if x >= 1 && y < size - 2 {
        v.push((x - 1, y + 2));
    }
    if x < size - 1 && y >= 2 {
        v.push((x + 1, y - 2));
    }
    if x < size - 1 && y < size - 2 {
        v.push((x + 1, y + 2));
    }
    if x < size - 2 && y >= 1 {
        v.push((x + 2, y - 1));
    }
    if x < size - 2 && y < size - 1 {
        v.push((x + 2, y + 1));
    }
    v
}

fn bfs(l: usize, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; l]; l];
    let mut distance = vec![vec![0; l]; l];
    queue.push_back(start);
    visited[start.0][start.1] = true;
    while let Some(point) = queue.pop_front() {
        if point == end {
            return distance[point.0][point.1];
        }
        for next in get_knight(l, point) {
            if !visited[next.0][next.1] {
                queue.push_back(next);
                visited[next.0][next.1] = true;
                distance[next.0][next.1] = distance[point.0][point.1] + 1;
            }
        }
    }
    unreachable!();
}

fn main() {
    let n = read_line_as_numbers()[0];
    for _ in 0..n {
        let l = read_line_as_numbers()[0];
        let start = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        let end = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        println!("{}", bfs(l, start, end));
    }
}
