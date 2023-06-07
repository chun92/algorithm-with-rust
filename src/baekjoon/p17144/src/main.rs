fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn _print_map(map: &Vec<Vec<i32>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{} ", map[i][j]);
        }
        println!();
    }
    println!();
}

fn print_sum(map: &Vec<Vec<i32>>) {
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == -1 {
                continue;
            }
            sum += map[i][j];
        }
    }
    println!("{}", sum);
}

fn main() {
    let (r, c, t) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1] as usize, v[2] as usize)
    };

    let mut map = vec![vec![0; c]; r];

    let mut cleaner = 0;

    for i in 0..r {
        let v = read_line_as_numbers();
        for j in 0..c {
            map[i][j] = v[j];

            if v[j] == -1 {
                if cleaner == 0 {
                    cleaner = i;
                }
            }
        }
    }

    for _ in 0..t {
        let mut next_board = vec![vec![0; c]; r];

        // Diffusion
        for i in 0..r {
            for j in 0..c {
                if map[i][j] == -1 {
                    next_board[i][j] = -1;
                    continue;
                }

                if map[i][j] == 0 {
                    continue;
                }

                let cur = map[i][j];
                let diffused = cur / 5;

                next_board[i][j] += map[i][j];

                if i > 0 && map[i - 1][j] != -1 {
                    next_board[i - 1][j] += diffused;
                    next_board[i][j] -= diffused;
                }

                if i < r - 1 && map[i + 1][j] != -1 {
                    next_board[i + 1][j] += diffused;
                    next_board[i][j] -= diffused;
                }

                if j > 0 && map[i][j - 1] != -1 {
                    next_board[i][j - 1] += diffused;
                    next_board[i][j] -= diffused;
                }

                if j < c - 1 && map[i][j + 1] != -1 {
                    next_board[i][j + 1] += diffused;
                    next_board[i][j] -= diffused;
                }
            }
        }

        // Clean
        let upper_cleaner = cleaner;
        let lower_cleaner = cleaner + 1;

        for i in (1..upper_cleaner).rev() {
            next_board[i][0] = next_board[i - 1][0];
        }

        for i in 0..c - 1 {
            next_board[0][i] = next_board[0][i + 1];
        }

        for i in 0..upper_cleaner {
            next_board[i][c - 1] = next_board[i + 1][c - 1];
        }

        for i in (1..c).rev() {
            next_board[upper_cleaner][i] = next_board[upper_cleaner][i - 1];
        }

        for i in lower_cleaner + 1..r - 1 {
            next_board[i][0] = next_board[i + 1][0];
        }

        for i in 0..c - 1 {
            next_board[r - 1][i] = next_board[r - 1][i + 1];
        }

        for i in (lower_cleaner + 1..r).rev() {
            next_board[i][c - 1] = next_board[i - 1][c - 1];
        }

        for i in (1..c).rev() {
            next_board[lower_cleaner][i] = next_board[lower_cleaner][i - 1];
        }

        next_board[upper_cleaner][1] = 0;
        next_board[lower_cleaner][1] = 0;

        map = next_board;
    }

    print_sum(&map);
}
