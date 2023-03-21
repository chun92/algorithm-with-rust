use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut vec_deque = (1..=n).collect::<VecDeque<i32>>();
    let mut result = 0;
    let vec_target = read_line_as_numbers();

    for i in 0..(m as usize) {
        if let Some((index, _)) = vec_deque
            .iter()
            .enumerate()
            .find(|&(_, &x)| x == vec_target[i]) {
                if index == 0 {
                    vec_deque.pop_front();
                } else if index <= vec_deque.len() / 2 {
                    for _ in 0..index {
                        let x = vec_deque.pop_front().unwrap();
                        vec_deque.push_back(x);
                        result += 1;
                    }
                    vec_deque.pop_front();
                } else {
                    for _ in 0..(vec_deque.len() - index) {
                        let x = vec_deque.pop_back().unwrap();
                        vec_deque.push_front(x);
                        result += 1;
                    }
                    vec_deque.pop_front();
                }
            }
    }
    println!("{}", result);
}
