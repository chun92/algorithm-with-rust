use std::io::stdin;
use std::collections::HashSet;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let arguments = read_line_as_numbers();
    let (_n, _m) = (arguments[0], arguments[1]);

    let a_set = read_line_as_numbers()
        .into_iter()
        .collect::<HashSet<i32>>();
    let b_set = read_line_as_numbers()
        .into_iter()
        .collect::<HashSet<i32>>();

    let a_b_diff_len = a_set.difference(&b_set).collect::<HashSet<_>>().len();
    let b_a_diff_len = b_set.difference(&a_set).collect::<HashSet<_>>().len();
    println!("{}", a_b_diff_len + b_a_diff_len);
}
