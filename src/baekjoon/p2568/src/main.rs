use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_number()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        let (a, b) = {
            let v = read_line_as_number();
            (v[0], v[1])
        };
        vec.push((a, b));
    }
    vec.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let vec = vec
        .iter()
        .map(|(a, _)| a)
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    let mut index = Vec::new();
    for i in 0..n {
        let num = vec[i];
        let idx = result.partition_point(|&x| x < num);
        if idx == result.len() {
            result.push(num);
        } else {
            result[idx] = num;
        }
        index.push(idx);
    }

    let mut count = result.len();
    let mut result = Vec::new();
    for i in (0..n).rev() {
        if count == 0 {
            break;
        } else {
            if index[i] == count - 1 {
                result.push(vec[i]);
                count -= 1;
            }
        }
    }
    result.reverse();

    let mut index = 0;
    let mut result_last = Vec::new();

    for i in 0..n {
        if index != result.len() && vec[i] == result[index] {
            index += 1;
        } else {
            result_last.push(vec[i]);
        }
    }

    result_last.sort_unstable();

    let mut output = BufWriter::new(stdout());
    writeln!(output, "{}", result_last.len()).unwrap();
    for i in 0..result_last.len() {
        writeln!(output, "{}", result_last[i]).unwrap();
    }
}
