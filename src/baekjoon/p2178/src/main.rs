fn read_line_as_numbers() -> Vec<i32> {
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

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut vec = Vec::new();
    for _ in 0..n {
        let nums: Vec<i32> = read_line_as_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        vec.push(nums);
    }

    let mut visited = vec![vec![false; m as usize]; n as usize];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0, 0, 1));

    while let Some((x, y, count)) = queue.pop_front() {
        if x == n - 1 && y == m - 1 {
            println!("{}", count);
            return;
        }

        if visited[x as usize][y as usize] {
            continue;
        }

        visited[x as usize][y as usize] = true;

        if x > 0 && vec[x as usize - 1][y as usize] == 1 {
            queue.push_back((x - 1, y, count + 1));
        }
        if x < n - 1 && vec[x as usize + 1][y as usize] == 1 {
            queue.push_back((x + 1, y, count + 1));
        }
        if y > 0 && vec[x as usize][y as usize - 1] == 1 {
            queue.push_back((x, y - 1, count + 1));
        }
        if y < m - 1 && vec[x as usize][y as usize + 1] == 1 {
            queue.push_back((x, y + 1, count + 1));
        }
    }
}
