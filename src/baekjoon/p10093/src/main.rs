use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (n, m) = {
        let line = read_line_as_numbers();
        let (x, y) = (line[0], line[1]);
        if x > y {
            (y, x)
        } else {
            (x, y)
        }
    };

    if n == m {
        println!("0");
        return;
    }
    
    let mut output = BufWriter::new(stdout());
    writeln!(output, "{}", m - n - 1).unwrap();
    for i in n + 1..m {
        if i != m - 1 {
            write!(output, "{} ", i).unwrap();
        } else {
            writeln!(output, "{}", i).unwrap();
        }
    }
}
