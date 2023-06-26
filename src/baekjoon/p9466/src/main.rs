use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut output = BufWriter::new(stdout());
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let n = read_line_as_numbers()[0];
        let mut vec = Vec::new();
        vec.push(0);
        let mut temp = read_line_as_numbers();
        vec.append(&mut temp);

        let mut visited = vec![0; n + 1];
        let mut index = 1;
        let mut stack = Vec::new();
        for i in 1..=n {
            if visited[i] != 0 {
                continue;
            }
            
            let mut current = i;
            loop {
                visited[current] = index;
                let next_value = vec[current];

                if current == next_value {
                    while let Some(x) = stack.pop() {
                        visited[x] = -1;
                    }
                    break;
                }
                
                stack.push(current);
                if visited[next_value] == index {
                    current = next_value;
                    visited[current] = index;
                    let mut found = false;
                    while let Some(x) = stack.pop() {
                        if found {
                            visited[x] = -1;
                        }

                        if x == next_value {
                            found = true;
                        }
                    }
                    break;
                } else if visited[next_value] != 0 {
                    while let Some(x) = stack.pop() {
                        visited[x] = -1;
                    }
                    break;
                } else {
                    current = next_value;
                }
            }
            index += 1;
        }

        let mut sum = 0;
        visited
            .iter()
            .for_each(|x| if *x == -1 { sum += 1 });
        writeln!(output, "{}", sum).unwrap();
    }
}
