use std::{io::{Stdout, stdin, stdout, Write, BufWriter}, collections::HashSet};

fn read_line_as_numbers() -> Vec<usize> {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn recursive(numbers: &Vec<usize>, output: &mut BufWriter<Stdout>, n: usize, m: usize, index: usize, result: &mut Vec<usize>) {
    if index == m {
        for i in 0..m {
            if i != 0 {
                write!(output, " ").unwrap();
            }
            write!(output, "{}", result[i]).unwrap();
        }
        writeln!(output).unwrap();
        return;
    }

    let mut visited = HashSet::new();
    for i in 0..n {
        if index > 0 && result[index - 1] > numbers[i] {
            continue;
        }
        if visited.contains(&numbers[i]) {
            continue;
        }
        visited.insert(numbers[i]);
        result.push(numbers[i]);
        recursive(numbers, output, n, m, index + 1, result);
        result.pop();
    }
}

fn main() {
    let (n, m) = {
        let args = read_line_as_numbers();
        (args[0], args[1])
    };

    let mut numbers = read_line_as_numbers();
    numbers.sort();

    let mut output = BufWriter::new(stdout());
    let mut result = Vec::new();
    recursive(&numbers, &mut output, n, m, 0, &mut result);
}
