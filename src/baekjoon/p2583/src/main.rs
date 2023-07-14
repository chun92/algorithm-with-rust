fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (h, w, n) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut board = vec![vec![0; w]; h];

    for _ in 0..n {
        let (x1, y1, x2, y2) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2], v[3])
        };

        let xmin = x1.min(x2);
        let xmax = x1.max(x2);
        let ymin = y1.min(y2);
        let ymax = y1.max(y2);

        for x in xmin..xmax {
            for y in ymin..ymax {
                board[y][x] += 1;
            }
        }
    }

    let mut visited = vec![vec![false; w]; h];
    let mut islands = Vec::new();

    for i in 0..h {
        for j in 0..w {
            if visited[i][j] || board[i][j] != 0 {
                continue;
            }
            let mut count = 0;

            let mut stack = Vec::new();
            stack.push((i, j));
            visited[i][j] = true;

            while let Some((x, y)) = stack.pop() {
                count += 1;

                if x > 0 && !visited[x - 1][y] && board[x - 1][y] == 0 {
                    stack.push((x - 1, y));
                    visited[x - 1][y] = true;
                }
                if x < h - 1 && !visited[x + 1][y] && board[x + 1][y] == 0 {
                    stack.push((x + 1, y));
                    visited[x + 1][y] = true;
                }
                if y > 0 && !visited[x][y - 1] && board[x][y - 1] == 0 {
                    stack.push((x, y - 1));
                    visited[x][y - 1] = true;
                }
                if y < w - 1 && !visited[x][y + 1] && board[x][y + 1] == 0 {
                    stack.push((x, y + 1));
                    visited[x][y + 1] = true;
                }
            }

            islands.push(count);
        }
    }

    islands.sort();

    println!("{}", islands.len());
    for island in islands {
        print!("{} ", island);
    }
}
