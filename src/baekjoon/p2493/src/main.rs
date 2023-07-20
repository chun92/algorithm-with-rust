fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut line = String::new();
    let mut stack = Vec::new();
    let mut result = Vec::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .enumerate()
        .for_each(|(i, s)| {
            let num = s.parse::<usize>().unwrap();
            let mut done = false;
            while let Some((index, value)) = stack.pop() {
                if value > num {
                    result.push(index);
                    stack.push((index, value));
                    stack.push((i + 1, num));
                    done = true;
                }

                if done {
                    break;
                }
            }

            if !done {
                result.push(0);
                stack.push((i + 1, num));
            }
    });

    for i in 0..n {
        print!("{} ", result[i]);
    }
}
