type Sudoku = Vec<Vec<Vec<usize>>>;
type Target = Vec<(usize, usize, usize)>;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_sudoku() -> Sudoku {
    let mut sudoku = Vec::new();
    for _ in 0..9 {
        let vec = read_line_as_numbers();
        let mut sudoku_row = Vec::new();
        for i in 0..9 {
            sudoku_row.push(vec![vec[i]]);
        }
        sudoku.push(sudoku_row);
    }
    sudoku
}

fn print_sudoku(sudoku: &Sudoku) {
    for row in sudoku {
        for number in row {
            for n in number {
                print!("{} ", n);
            }
        }
        println!();
    }
}

fn init_sudoku(sudoku: &mut Sudoku) {
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j].contains(&0) {
                get_possible_nubmer(i, j, sudoku);
            }
        }
    }
}

fn get_possible_nubmer(i: usize, j: usize, sudoku: &mut Sudoku) {
    let mut possible_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for k in 0..9 {
        if sudoku[i][k].len() == 1 && sudoku[i][k][0] != 0 {
            possible_numbers.retain(|&x| x != sudoku[i][k][0]);
        }
        if sudoku[k][j].len() == 1 && sudoku[k][j][0] != 0 {
            possible_numbers.retain(|&x| x != sudoku[k][j][0]);
        }
    }

    let x = i / 3 * 3;
    let y = j / 3 * 3;
    for k in x..x + 3 {
        for l in y..y + 3 {
            if sudoku[k][l].len() == 1 && sudoku[k][l][0] != 0 {
                possible_numbers.retain(|&x| x != sudoku[k][l][0]);
            }
        }
    }

    sudoku[i][j] = possible_numbers.clone();
}

fn check_remaining_valid(i: usize, j: usize, remanining: usize, sudoku: &Sudoku) -> bool {
    for k in 0..9 {
        if j != k {
            if sudoku[i][k].len() == 1 && sudoku[i][k][0] == remanining {
                return false;
            }
        }
        if i != k {
            if sudoku[k][j].len() == 1 && sudoku[k][j][0] == remanining {
                return false;
            }
        }
    }

    let x = i / 3 * 3;
    let y = j / 3 * 3;
    for k in x..x + 3 {
        for l in y..y + 3 {
            if i != k && j != l  {
                if sudoku[k][l].len() == 1 && sudoku[k][l][0] == remanining{
                    return false;
                }
            }
        }
    }

    return true;
}

fn solve_sudoku_done(target_vec: &Sudoku) -> (bool, bool, Target) {
    let mut result_vec = Vec::new();
    let mut res = true;
    for i in 0..9 {
        for j in 0..9 {
            let len = target_vec[i][j].len();
            if len == 0 {
                return (false, false, result_vec);
            } else if len != 1 {
                res = false;
                result_vec.push((i, j, len));
            }
        }
    }

    return (true, res, result_vec);
}

fn candidate_one(i: usize, j: usize, removing_num: usize, sudoku: &mut Sudoku, vec: &Target) -> bool {
    for (x, y, _) in vec {
        if *x == i && *y == j {
            sudoku[*x][*y] = vec![removing_num];
        } else if *x == i || *y == j || (*x / 3 == i / 3 && *y / 3 == j / 3) {
            if sudoku[*x][*y].contains(&removing_num) {
                sudoku[*x][*y].retain(|&v| v != removing_num);
            }
        }
    }

    for (x, y, _) in vec {
        if sudoku[*x][*y].len() == 1 {
            if !check_remaining_valid(*x, *y, sudoku[*x][*y][0], sudoku) {
                return false;
            }
        }
    }

    return true;
}

fn solve_sudoku(sudoku: &mut Sudoku) -> bool {
    let result = solve_sudoku_done(&sudoku);
    
    if result.0 == false {
        return false;
    }

    if result.1 == true {
        return true;
    }
    
    let mut target_vec = result.2;

    target_vec.sort_by(|a, b| a.2.cmp(&b.2));

    let (i, j) = (target_vec[0].0, target_vec[0].1);
    let mut removing_nums = sudoku[i][j].clone();

    while removing_nums.len() > 0 {
        let mut result_sudoku = sudoku.clone();
        let candidate_success = candidate_one(i, j, removing_nums[0], &mut result_sudoku, &target_vec);
        if candidate_success && solve_sudoku(&mut result_sudoku) {
            *sudoku = result_sudoku;
            return true;
        } else {
            removing_nums.remove(0);
            sudoku[i][j].remove(0);
        }
    }

    return false;
}

fn main() {
    let mut sudoku = get_sudoku();
    init_sudoku(&mut sudoku);
    solve_sudoku(&mut sudoku);
    print_sudoku(&sudoku);
}
