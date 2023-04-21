fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}


fn check_tetriminos(board: &Vec<Vec<usize>>, n: usize, m: usize) -> usize {
    // *
    // *
    // *
    // *
    fn check_tetrimino_i_1(board: &Vec<Vec<usize>>, n: usize, _m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 4 {
            None
        } else {
            Some(board[i][j] + board[i+1][j] + board[i+2][j] + board[i+3][j])
        }
    }

    // ****
    fn check_tetrimino_i_2(board: &Vec<Vec<usize>>, _n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if j > m - 4 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i][j+2] + board[i][j+3])
        }
    }

    // **
    // **
    fn check_tetrimino_o(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 2 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i+1][j] + board[i+1][j+1])
        }
    }

    // *
    // *
    // **
    fn check_tetrimino_l_1(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j] + board[i+1][j] + board[i+2][j] + board[i+2][j+1])
        }
    }

    //   *
    // ***
    fn check_tetrimino_l_2(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i+1][j] + board[i+1][j+1] + board[i+1][j+2] + board[i][j+2])
        }
    }

    // **
    //  *
    //  *
    fn check_tetrimino_l_3(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i+1][j+1] + board[i+2][j+1])
        }
    }

    // ***
    // *
    fn check_tetrimino_l_4(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i][j+2] + board[i+1][j])
        }
    }

    //  *
    //  *
    // **
    fn check_tetrimino_l_5(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j+1] + board[i+1][j+1] + board[i+2][j] + board[i+2][j+1])
        }
    }

    // *
    // ***
    fn check_tetrimino_l_6(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j] + board[i+1][j] + board[i+1][j+1] + board[i+1][j+2])
        }
    }

    // **
    // *
    // *
    fn check_tetrimino_l_7(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i+1][j] + board[i+2][j])
        }
    }

    // ***
    //   *
    fn check_tetrimino_l_8(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i][j+2] + board[i+1][j+2])
        }
    }

    // **
    //  **
    fn check_tetrimino_z_1(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i+1][j+1] + board[i+1][j+2])
        }
    }

    //  *
    // **
    // *
    fn check_tetrimino_z_2(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j+1] + board[i+1][j] + board[i+1][j+1] + board[i+2][j])
        }
    }

    //  **
    // **
    fn check_tetrimino_z_3(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j+1] + board[i][j+2] + board[i+1][j] + board[i+1][j+1])
        }
    }

    // *
    // **
    //  *
    fn check_tetrimino_z_4(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j] + board[i+1][j] + board[i+1][j+1] + board[i+2][j+1])
        }
    }

    //  *
    // ***
    fn check_tetrimino_t_1(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j+1] + board[i+1][j] + board[i+1][j+1] + board[i+1][j+2])
        }
    }

    // *
    // **
    // *
    fn check_tetrimino_t_2(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j] + board[i+1][j] + board[i+1][j+1] + board[i+2][j])
        }
    }

    // ***
    //  *
    fn check_tetrimino_t_3(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 2 || j > m - 3 {
            None
        } else {
            Some(board[i][j] + board[i][j+1] + board[i][j+2] + board[i+1][j+1])
        }
    }

    //  *
    // **
    //  *
    fn check_tetrimino_t_4(board: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> Option<usize> {
        if i > n - 3 || j > m - 2 {
            None
        } else {
            Some(board[i][j+1] + board[i+1][j] + board[i+1][j+1] + board[i+2][j+1])
        }
    }

    let mut tetriminos: Vec<fn(&Vec<Vec<usize>>, usize, usize, usize, usize) -> Option<usize>> = Vec::new();
    tetriminos.push(check_tetrimino_i_1);
    tetriminos.push(check_tetrimino_i_2);
    tetriminos.push(check_tetrimino_o);
    tetriminos.push(check_tetrimino_l_1);
    tetriminos.push(check_tetrimino_l_2);
    tetriminos.push(check_tetrimino_l_3);
    tetriminos.push(check_tetrimino_l_4);
    tetriminos.push(check_tetrimino_l_5);
    tetriminos.push(check_tetrimino_l_6);
    tetriminos.push(check_tetrimino_l_7);
    tetriminos.push(check_tetrimino_l_8);
    tetriminos.push(check_tetrimino_z_1);
    tetriminos.push(check_tetrimino_z_2);
    tetriminos.push(check_tetrimino_z_3);
    tetriminos.push(check_tetrimino_z_4);
    tetriminos.push(check_tetrimino_t_1);
    tetriminos.push(check_tetrimino_t_2);
    tetriminos.push(check_tetrimino_t_3);
    tetriminos.push(check_tetrimino_t_4);

    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            for tetrimino in &tetriminos {
                if let Some(sum) = tetrimino(&board, n, m, i, j) {
                    if sum > max {
                        max = sum;
                    }
                }
            }
        }
    }
    max
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut board = Vec::new();
    for _ in 0..n {
        let v = read_line_as_numbers();
        board.push(v);
    }

    println!("{}", check_tetriminos(&board, n, m));
}
