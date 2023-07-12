fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn check_islands(map: &Vec<Vec<usize>>, w: usize, h: usize) -> usize {
    let mut visited = vec![vec![false; w]; h];
    let mut count = 0;

    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                continue;
            }

            if map[i][j] == 0 {
                continue;
            }

            let mut stack = Vec::new();
            stack.push((i, j));

            while let Some((i, j)) = stack.pop() {
                visited[i][j] = true;

                if i > 0 && !visited[i - 1][j] && map[i - 1][j] == 1 {
                    stack.push((i - 1, j));
                }

                if i < h - 1 && !visited[i + 1][j] && map[i + 1][j] == 1 {
                    stack.push((i + 1, j));
                }

                if j > 0 && !visited[i][j - 1] && map[i][j - 1] == 1 {
                    stack.push((i, j - 1));
                }

                if j < w - 1 && !visited[i][j + 1] && map[i][j + 1] == 1 {
                    stack.push((i, j + 1));
                }

                if i > 0 && j > 0 && !visited[i - 1][j - 1] && map[i - 1][j - 1] == 1 {
                    stack.push((i - 1, j - 1));
                }

                if i > 0 && j < w - 1 && !visited[i - 1][j + 1] && map[i - 1][j + 1] == 1 {
                    stack.push((i - 1, j + 1));
                }

                if i < h - 1 && j > 0 && !visited[i + 1][j - 1] && map[i + 1][j - 1] == 1 {
                    stack.push((i + 1, j - 1));
                }

                if i < h - 1 && j < w - 1 && !visited[i + 1][j + 1] && map[i + 1][j + 1] == 1 {
                    stack.push((i + 1, j + 1));
                }
            }
            count += 1;
        }
    }

    count
}

fn main() {
    loop {
        let (w, h) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };

        if w == 0 && h == 0 {
            break;
        }

        let mut map = vec![vec![0; w]; h];
        for i in 0..h {
            let v = read_line_as_numbers();
            for j in 0..w {
                map[i][j] = v[j];
            }
        }

        println!("{}", check_islands(&map, w, h));
    }
}
