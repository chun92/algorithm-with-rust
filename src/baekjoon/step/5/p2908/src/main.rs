use std::io;

fn read_line_as_swap_and_print_numbers() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let vec:Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| reverse_str(x)
            .parse::<i32>()
            .unwrap())
        .collect();
    let result = std::cmp::max(vec[0], vec[1]);
    println!("{}", result);
}

fn reverse_str(target: &str) -> String {
    let string = String::from(target);
    let result = string
        .chars()
        .rev()
        .collect();
    result
}

fn main() {
    read_line_as_swap_and_print_numbers();
}
