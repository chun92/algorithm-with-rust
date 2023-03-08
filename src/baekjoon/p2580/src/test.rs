

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_sudoku() -> Vec<Vec<usize>> {
    let mut sudoku = Vec::new();
    for _ in 0..9 {
        sudoku.push(read_line_as_numbers());
    }
    sudoku
}

fn print_sudoku(sudoku: &Vec<Vec<usize>>) {
    for row in sudoku {
        for number in row {
            print!("{} ", number);
        }
        println!();
    }
}

fn get_possible_nubmer(i: usize, j: usize, sudoku: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut possible_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for k in 0..9 {
        if sudoku[i][k] != 0 {
            possible_numbers.retain(|&x| x != sudoku[i][k]);
        }
        if sudoku[k][j] != 0 {
            possible_numbers.retain(|&x| x != sudoku[k][j]);
        }
    }

    let i = i / 3 * 3;
    let j = j / 3 * 3;
    for k in i..i + 3 {
        for l in j..j + 3 {
            if sudoku[k][l] != 0 {
                possible_numbers.retain(|&x| x != sudoku[k][l]);
            }
        }
    }

    possible_numbers
}

fn get_zeros_in_sudoku(sudoku: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut zeros = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == 0 {
                zeros.push((i, j));
            }
        }
    }
    zeros
}

fn fill_trivials(sudoku: &mut Vec<Vec<usize>>) -> usize {
    let mut zeros = get_zeros_in_sudoku(&sudoku);
    let mut prev_len = 0;
    while zeros.len() > 0 {
        if prev_len == zeros.len() {
            break;
        }
        prev_len = zeros.len();
        for (i, j) in zeros {
            let possible_numbers = get_possible_nubmer(i, j, &sudoku);
            if possible_numbers.len() == 1 {
                sudoku[i][j] = possible_numbers[0];
            }
        }
        zeros = get_zeros_in_sudoku(&sudoku);
    }
    zeros.len()
}

fn solve_sudoku(sudoku: &mut Vec<Vec<usize>>) -> bool {
    let zeros = fill_trivials(sudoku);
    if zeros == 0 {
        return true;
    }

    let mut zeros = get_zeros_in_sudoku(&sudoku);
    zeros.sort_by(|a, b| {
        get_possible_nubmer(a.0, a.1, &sudoku)
            .len()
            .cmp(&get_possible_nubmer(b.0, b.1, &sudoku).len())
    });

    let (i, j) = zeros[0];
    let possible_numbers = get_possible_nubmer(i, j, &sudoku);
    for number in possible_numbers {
        let mut sudoku_inner = sudoku.clone();
        sudoku_inner[i][j] = number;
        if solve_sudoku(&mut sudoku_inner) {
            *sudoku = sudoku_inner;
            return true;
        }
    }
    sudoku[i][j] = 0;
    false
}

fn main() {
    let mut sudoku = get_sudoku();
    solve_sudoku(&mut sudoku);
    print_sudoku(&sudoku);
}