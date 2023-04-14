use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let in_empty = n - 2;
    let mid_empty = 2 * n - 3;

    let mut output = BufWriter::new(stdout());

    vec!['*'; n].iter().for_each(|c| write!(output, "{}", c).unwrap());
    vec![' '; mid_empty].iter().for_each(|c| write!(output, "{}", c).unwrap());
    vec!['*'; n].iter().for_each(|c| write!(output, "{}", c).unwrap());
    writeln!(output).unwrap();
    
    for i in -(n as i32 - 2)..=(n as i32 - 2) {
        let abs_i = i.abs() as usize;
        let indent = n - 1 - abs_i;
        let mid_empty = if i == 0 { 0 } else { mid_empty - 2 * indent };
        
        vec![' '; indent].iter().for_each(|c| write!(output, "{}", c).unwrap());
        write!(output, "*").unwrap();
        vec![' '; in_empty].iter().for_each(|c| write!(output, "{}", c).unwrap());
        if i == 0 {
            write!(output, "*").unwrap();
        } else {
            write!(output, "*").unwrap();
            vec![' '; mid_empty].iter().for_each(|c| write!(output, "{}", c).unwrap());
            write!(output, "*").unwrap();
        }
        vec![' '; in_empty].iter().for_each(|c| write!(output, "{}", c).unwrap());
        write!(output, "*").unwrap();
        writeln!(output).unwrap();
    }
    
    vec!['*'; n].iter().for_each(|c| write!(output, "{}", c).unwrap());
    vec![' '; mid_empty].iter().for_each(|c| write!(output, "{}", c).unwrap());
    vec!['*'; n].iter().for_each(|c| write!(output, "{}", c).unwrap());
    writeln!(output).unwrap();
}
