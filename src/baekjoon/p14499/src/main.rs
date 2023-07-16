fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m, x, y, _k) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2], v[3], v[4])
    };

    let mut board = vec![vec![0; m]; n];
    for i in 0..n {
        let v = read_line_as_numbers();
        for j in 0..m {
            board[i][j] = v[j];
        }
    }

    let mut dice = vec![0; 6];
    let commands = read_line_as_numbers();
    let mut x = x;
    let mut y = y;

    for command in commands {
        let x1 = x as i32;
        let y1 = y as i32;
        let (nx, ny) = match command {
            1 => (x1, y1 + 1),
            2 => (x1, y1 - 1),
            3 => (x1 - 1, y1),
            4 => (x1 + 1, y1),
            _ => unreachable!(),
        };

        if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
            continue;
        }

        x = nx as usize;
        y = ny as usize;

        let mut new_dice = vec![0; 6];
        match command {
            1 => {
                new_dice[0] = dice[3];
                new_dice[1] = dice[1];
                new_dice[2] = dice[0];
                new_dice[3] = dice[5];
                new_dice[4] = dice[4];
                new_dice[5] = dice[2];
            }
            2 => {
                new_dice[0] = dice[2];
                new_dice[1] = dice[1];
                new_dice[2] = dice[5];
                new_dice[3] = dice[0];
                new_dice[4] = dice[4];
                new_dice[5] = dice[3];
            }
            3 => {
                new_dice[0] = dice[4];
                new_dice[1] = dice[0];
                new_dice[2] = dice[2];
                new_dice[3] = dice[3];
                new_dice[4] = dice[5];
                new_dice[5] = dice[1];
            }
            4 => {
                new_dice[0] = dice[1];
                new_dice[1] = dice[5];
                new_dice[2] = dice[2];
                new_dice[3] = dice[3];
                new_dice[4] = dice[0];
                new_dice[5] = dice[4];
            }
            _ => unreachable!(),
        }

        dice = new_dice;

        if board[x][y] == 0 {
            board[x][y] = dice[5];
        } else {
            dice[5] = board[x][y];
            board[x][y] = 0;
        }

        println!("{}", dice[0]);
    }
}
