use std::io::{stdin, stdout, BufWriter, Write, Stdout};

fn read_line_as_number() -> u8 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn permutation(output: &mut BufWriter<Stdout>, remain: &Vec<u8>, current: &Vec<u8>) {
    if remain.len() == 0 {
        for i in current {
            write!(output, "{} ", i).unwrap();
        }
        writeln!(output).unwrap();
        return;
    }

    for s in remain {
        let mut new_remain = remain.clone();
        new_remain.retain(|&x| x != *s);
        let mut new_current = current.clone();
        new_current.push(*s);
        permutation(output, &new_remain, &new_current);
    }
}

fn main() {
    let n = read_line_as_number();
    let vec = (1..=n).collect::<Vec<u8>>();
    let mut output = BufWriter::new(stdout());

    permutation(&mut output, &vec, &vec![]);
}
