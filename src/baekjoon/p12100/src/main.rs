fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn move_line(line: &mut Vec<usize>) {
    let mut cur = 0;
    let mut cur_index = 0;
    for i in 0..line.len() {
        let cur_num = line[i];
        line[i] = 0;
        if cur_num == 0 {
            continue;
        }

        if cur == 0 {
            cur = cur_num;
            line[cur_index] = cur;
        } else {
            if cur == cur_num {
                cur *= 2;
                line[cur_index] = cur;
                cur = 0;
                cur_index += 1;
            } else {
                cur = cur_num;
                cur_index += 1;
                line[cur_index] = cur;
            }
        }
    }
}

// 0: up, 1: right, 2: down, 3: left
fn move_board(direction: usize, board: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, usize) {
    let mut board = board.clone();
    let mut max = 0;

    if direction == 0 {
        for j in 0..board.len() {
            let mut line = Vec::new();
            for i in 0..board.len() {
                line.push(board[i][j]);
            }
            move_line(&mut line);
            for i in 0..board.len() {
                board[i][j] = line[i];
                if line[i] > max {
                    max = line[i];
                }
            }
        }
    } else if direction == 1 {
        for i in 0..board.len() {
            let mut line = board[i].iter().rev().map(|&x| x).collect();
            move_line(&mut line);
            board[i] = line.iter().rev().map(|&x| x).collect();
            let line_max = line.iter().max().unwrap();
            if *line_max > max {
                max = *line_max;
            }
        }
    } else if direction == 2 {
        for j in 0..board.len() {
            let mut line = Vec::new();
            for i in (0..board.len()).rev() {
                line.push(board[i][j]);
            }
            move_line(&mut line);
            line = line.iter().rev().map(|&x| x).collect();
            for i in 0..board.len() {
                board[i][j] = line[i];
                if line[i] > max {
                    max = line[i];
                }
            }
        }
    } else if direction == 3 {
        for i in 0..board.len() {
            let mut line = board[i].clone();
            move_line(&mut line);
            let line_max = line.iter().max().unwrap();
            if *line_max > max {
                max = *line_max;
            }
            board[i] = line;
        }
    }
    
    (board, max)
}

const MAX_COUNT: usize = 5;

fn solve(board: &Vec<Vec<usize>>) -> usize {
    bruteforce(0, board, 0)
}

fn bruteforce(count: usize, board: &Vec<Vec<usize>>, max: usize) -> usize {
    if count == MAX_COUNT {
        return max;
    }

    let mut max = max;
    for i in 0..4 {
        let (new_board, new_max) = move_board(i, board);
        let new_max = bruteforce(count + 1, &new_board, new_max);
        if new_max > max {
            max = new_max;
        }
    }

    max
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut board = Vec::new();
    for _ in 0..n {
        board.push(read_line_as_numbers());
    }

    println!("{}", solve(&board));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_line_1() {
        let mut line = vec![2, 2, 0, 0];
        move_line(&mut line);
        assert_eq!(line, vec![4, 0, 0, 0]);
    }

    #[test]
    fn test_move_line_2() {
        let mut line = vec![2, 4, 2, 0];
        move_line(&mut line);
        assert_eq!(line, vec![2, 4, 2, 0]);
    }

    #[test]
    fn test_move_line_3() {
        let mut line = vec![0, 0, 4, 0];
        move_line(&mut line);
        assert_eq!(line, vec![4, 0, 0, 0]);
    }

    #[test]
    fn test_move_line_4() {
        let mut line = vec![2, 2, 2, 2];
        move_line(&mut line);
        assert_eq!(line, vec![4, 4, 0, 0]);
    }

    #[test]
    fn test_move_line_5() {
        let mut line = vec![2, 2, 2, 4];
        move_line(&mut line);
        assert_eq!(line, vec![4, 2, 4, 0]);
    }

    #[test]
    fn test_move_line_6() {
        let mut line = vec![0, 2, 2, 2, 0, 2, 2, 2];
        move_line(&mut line);
        assert_eq!(line, vec![4, 4, 4, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_move_line_7() {
        let mut line = vec![2, 2, 4, 4];
        move_line(&mut line);
        assert_eq!(line, vec![4, 8, 0, 0]);
    }

    #[test]
    fn test_move_board_up_1() {
        let board = vec![
            vec![2, 2, 0, 0],
            vec![2, 2, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(0, &board).0;
        assert_eq!(
            board,
            vec![
                vec![4, 4, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_up_2() {
        let board = vec![
            vec![2, 2, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(0, &board).0;
        assert_eq!(
            board,
            vec![
                vec![2, 4, 0, 0],
                vec![4, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_up_3() {
        let board = vec![
            vec![0, 4, 0, 0],
            vec![4, 0, 2, 0],
            vec![2, 4, 2, 0],
            vec![4, 0, 4, 0],
        ];
        let board = move_board(0, &board).0;
        assert_eq!(
            board,
            vec![
                vec![4, 8, 4, 0],
                vec![2, 0, 4, 0],
                vec![4, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }


    #[test]
    fn test_move_board_up_4() {
        let board = vec![
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![4, 0, 0, 0],
        ];
        let board = move_board(0, &board).0;
        assert_eq!(
            board,
            vec![
                vec![4, 0, 0, 0],
                vec![8, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_right_1() {
        let board = vec![
            vec![2, 2, 0, 0],
            vec![2, 2, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(1, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 4],
                vec![0, 0, 0, 4],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_right_2() {
        let board = vec![
            vec![2, 2, 0, 0],
            vec![2, 4, 0, 2],
            vec![0, 0, 4, 4],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(1, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 4],
                vec![0, 2, 4, 2],
                vec![0, 0, 0, 8],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_down_1() {
        let board = vec![
            vec![2, 2, 0, 0],
            vec![2, 2, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(2, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![4, 4, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_down_2() {
        let board = vec![
            vec![2, 4, 0, 2],
            vec![4, 2, 4, 2],
            vec![2, 4, 0, 4],
            vec![0, 0, 4, 4],
        ];
        let board = move_board(2, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 0],
                vec![2, 4, 0, 0],
                vec![4, 2, 0, 4],
                vec![2, 4, 8, 8],
            ]
        );
    }

    
    #[test]
    fn test_move_board_down_3() {
        let board = vec![
            vec![2, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![4, 0, 0, 0],
        ];
        let board = move_board(2, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![4, 0, 0, 0],
                vec![8, 0, 0, 0],
            ]
        );
    }


    #[test]
    fn test_move_board_left_1() {
        let board = vec![
            vec![2, 4, 0, 2],
            vec![4, 2, 4, 2],
            vec![2, 4, 0, 4],
            vec![0, 0, 4, 4],
        ];
        let board = move_board(3, &board).0;
        assert_eq!(
            board,
            vec![
                vec![2, 4, 2, 0],
                vec![4, 2, 4, 2],
                vec![2, 8, 0, 0],
                vec![8, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_left_2() {
        let board = vec![
            vec![2, 2, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 4],
            vec![0, 16, 4, 4],
        ];
        let board = move_board(3, &board).0;
        assert_eq!(
            board,
            vec![
                vec![4, 0, 0, 0],
                vec![4, 2, 0, 0],
                vec![4, 0, 0, 0],
                vec![16, 8, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_example_1() {
        let board = vec![
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(0, &board).0;
        assert_eq!(
            board,
            vec![
                vec![2, 0, 2, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_example_2() {
        let board = vec![
            vec![2, 0, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(3, &board).0;
        assert_eq!(
            board,
            vec![
                vec![4, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_example_4() {
        let board = vec![
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
        ];
        let board = move_board(1, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 4, 2],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 2],
            ]
        );
    }

    #[test]
    fn test_move_board_example_5() {
        let board = vec![
            vec![0, 0, 4, 2],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 2],
        ];
        let board = move_board(0, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 4, 4],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_move_board_example_6() {
        let board = vec![
            vec![0, 0, 4, 4],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let board = move_board(1, &board).0;
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0, 8],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_solve_1() {
        let board = vec![
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(solve(&board), 4);
    }

    #[test]
    fn test_solve_2() {
        let board = vec![
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(solve(&board), 8);
    }


    #[test]
    fn test_solve_3() {
        let board = vec![
            vec![2, 0, 2, 8],
            vec![0, 0, 2, 2],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(solve(&board), 16);
    }

    #[test]
    fn test_solve_4() {
        let board = vec![
            vec![2, 2, 2],
            vec![4, 4, 4],
            vec![8, 8, 8],
        ];

        assert_eq!(solve(&board), 16);
    }
}
