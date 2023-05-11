use std::collections::VecDeque;

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn main() {
    let n = read_line_as_number();

    let mut prev_vec = vec![0; 10000001];

    let mut queue = VecDeque::new();
    let mut distance = 0;

    queue.push_back(n);

    'outer: while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while let Some(x) = queue.pop_front() {
            if x == 1 {
                break 'outer;
            }

            if x % 3 == 0 && prev_vec[x / 3] == 0 {
                prev_vec[x / 3] = x;
                next_queue.push_back(x / 3);
            }

            if x % 2 == 0 && prev_vec[x / 2] == 0 {
                prev_vec[x / 2] = x;
                next_queue.push_back(x / 2);
            }

            if prev_vec[x - 1] == 0 {
                prev_vec[x - 1] = x;
                next_queue.push_back(x - 1);
            }
        }
        distance += 1;
        queue = next_queue;
    }

    println!("{}", distance);
    
    let mut result = Vec::new();
    let mut index = 1;
    loop {
        result.push(index);
        if index == n {
            break;
        } else {
            index = prev_vec[index];
        }
    }

    let result =
        result.into_iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
}