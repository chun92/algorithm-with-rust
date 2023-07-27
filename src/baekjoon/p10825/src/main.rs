use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let n = read_line_as_strings()[0].parse::<usize>().unwrap();
    let mut vec = Vec::new();
    for _ in 0..n {
        let v = read_line_as_strings();
        vec.push((String::from(&v[0]), 
            v[1].parse::<usize>().unwrap(), 
            v[2].parse::<usize>().unwrap(), 
            v[3].parse::<usize>().unwrap()));
    }

    vec.sort_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else if a.2 != b.2 {
            a.2.cmp(&b.2)
        } else if a.3 != b.3 {
            b.3.cmp(&a.3)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut output = BufWriter::new(stdout());
    for v in vec {
        writeln!(output, "{}", v.0).unwrap();
    }
}
