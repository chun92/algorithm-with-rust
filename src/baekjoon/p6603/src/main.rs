use std::io::{stdin, stdout, Write, BufWriter, Stdout};

fn print_permutation(output: &mut BufWriter<Stdout> , vec: &mut Vec<usize>, n: usize) {
    vec.sort_unstable();

    let len = vec.len();
    let mut indexes = (0..n).collect::<Vec<usize>>();

    loop {
        for i in 0..n {
            write!(output, "{} ", vec[indexes[i]]).unwrap();
        }
        writeln!(output).unwrap();

        if indexes[0] == len - n {
            break;
        }

        let mut i = n - 1;

        while indexes[i] == len - n + i {
            i -= 1;
        }

        let mut cur = indexes[i] + 1;
        while i < n {
            indexes[i] = cur;
            cur += 1;
            i += 1;
        }
    }
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn main() {
    let mut output = BufWriter::new(stdout());

    let mut count = 0;
    loop {
        let line = read_line_as_numbers();
        if line[0] == 0 {
            break;
        }

        if count > 0 {
            writeln!(output).unwrap();
        }

        let n = line[0];
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = line[i + 1];
        }

        print_permutation(&mut output, &mut vec, 6);

        count += 1;
    }
}
