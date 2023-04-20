use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn do_dslr(command: u8, number: i32) -> i32 {
    match command {
        0 => (number * 2) % 10000,
        1 => if number == 0 { 9999 } else { number - 1 },
        2 => (number % 1000) * 10 + number / 1000,
        3 => (number % 10) * 1000 + number / 10,
        _ => unreachable!(),
    }
}

fn solve(source: i32, target: i32) -> String {
    let mut queue = VecDeque::new();
    let mut is_visited = vec![false; 10000];
    let mut prev_command = vec![(-1, 5); 10000];
    queue.push_back(source);
    is_visited[source as usize] = true;

    while let Some(number) = queue.pop_front() {
        if number == target {
            let mut current = number;
            let mut result = String::new();
            loop {
                if current == source {
                    return result.chars().rev().collect();
                }
                let (prev, command) = &prev_command[current as usize];
                current = *prev;
                result.push(match command {
                    0 => 'D',
                    1 => 'S',
                    2 => 'L',
                    3 => 'R',
                    _ => unreachable!(),
                });
            }
        }

        let mut next = |command: u8| {
            let next = do_dslr(command, number);
            if !is_visited[next as usize] {
                is_visited[next as usize] = true;
                prev_command[next as usize] = (number, command);
                queue.push_back(next);
            }
        };

        next(0);
        next(1);
        next(2);
        next(3);
    }

    unreachable!();
}

fn main() {
    let n = read_line_as_numbers()[0];
    for _ in 0..n {
        let numbers = read_line_as_numbers();
        let result = solve(numbers[0], numbers[1]);
        println!("{}", result);
    }
}
