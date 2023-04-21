use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<usize> {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (n, m) = {
        let args = read_line_as_numbers();
        (args[0], args[1])
    };

    let mut numbers = read_line_as_numbers();
    numbers.sort();

    let mut output = BufWriter::new(stdout());
    let mut current = m - 1;
    let mut indexes = vec![0; m];
    'outer: loop {
        let str: String = indexes
            .iter()
            .map(|i| numbers[*i].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        
        writeln!(output, "{}", str).unwrap();

        while indexes[current] == n - 1 {
            if current != 0 {
                current -= 1;
            } else {
                break 'outer;
            }
        }

        indexes[current] += 1;
        for i in current + 1..m {
            indexes[i] = indexes[i - 1];
        }
        current = m - 1;
    }
}
