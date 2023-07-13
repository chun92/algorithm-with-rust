fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn _print(x: usize, y: usize, d: usize, count: usize, board: &Vec<Vec<usize>>) {
    println!("x: {}, y: {}, d: {}, count: {}", x, y, d, count);
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("{}", board[i][j]);
        }
        println!();
    }
    println!();
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let (mut x, mut y, mut d) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut board = vec![vec![0; m]; n];

    for i in 0..n {
        let v = read_line_as_numbers();
        for j in 0..m {
            board[i][j] = v[j];
        }
    }

    let mut count = 0;

    loop {
        // print(x, y, d, count, &board);
        if board[x][y] == 0 {
            board[x][y] = 2;
            count += 1;
            continue;
        }

// <- 북쪽
// v 동쪽
// -> 남쪽
// ^ 서쪽

        let mut check_block = vec![1; 4];
        check_block[0] = board[x - 1][y];
        check_block[1] = board[x][y + 1];
        check_block[2] = board[x + 1][y];
        check_block[3] = board[x][y - 1];
        if check_block.iter().all(|&v| v != 0) {
            match d {
                0 => {
                    if check_block[2] == 1 {
                        break;
                    } else {
                        x += 1;
                    }
                },
                1 => {
                    if check_block[3] == 1 {
                        break;
                    } else {
                        y -= 1;
                    }
                },
                2 => {
                    if check_block[0] == 1 {
                        break;
                    } else {
                        x -= 1;
                    }
                },
                3 => {
                    if check_block[1] == 1 {
                        break;
                    } else {
                        y += 1;
                    }
                },
                _ => panic!("Invalid Direction")
            }
        } else {
            loop {
                d = (d + 3) % 4;
                match d {
                    0 => {
                        if check_block[0] == 0 {
                            x -= 1;
                            break;
                        }
                    },
                    1 => {
                        if check_block[1] == 0 {
                            y += 1;
                            break;
                        }
                    },
                    2 => {
                        if check_block[2] == 0 {
                            x += 1;
                            break;
                        }
                    },
                    3 => {
                        if check_block[3] == 0 {
                            y -= 1;
                            break;
                        }
                    },
                    _ => panic!("Invalid Direction")
                }
            }
        }
    }
    println!("{}", count);
}
