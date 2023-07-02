#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let (n, m) = {
        let input = read_line_as_numbers();
        (input[0], input[1])
    };

    let mut map = Vec::new();

    for _ in 0..n {
        let input = read_line_as_string();
        let vec = input
            .chars()
            .map(|x| match x {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid input")
            })
            .collect::<Vec<Direction>>();
        map.push(vec);
    }

    let mut indexes = vec![vec![0; m]; n];
    let mut current_index = 1;
    let mut stack = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if indexes[i][j] != 0 {
                continue;
            }
            let mut i = i;
            let mut j = j;

            stack.push((i, j));
            indexes[i][j] = current_index;
            
            let target_index;
            loop {
                let direction = &map[i][j];
                match direction {
                    Direction::Up => i -= 1,
                    Direction::Down => i += 1,
                    Direction::Left => j -= 1,
                    Direction::Right => j += 1
                }

                if indexes[i][j] != 0 {
                    target_index = indexes[i][j];
                    break;
                } else {
                    stack.push((i, j));
                    indexes[i][j] = current_index;
                }
            }
            
            while let Some((i, j)) = stack.pop() {
                indexes[i][j] = target_index;
            }

            if target_index == current_index {
                current_index += 1;
            }
        }
    }

    println!("{}", current_index - 1);
}
