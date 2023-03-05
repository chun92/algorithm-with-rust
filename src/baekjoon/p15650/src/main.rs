use std::fmt::Write;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn print_num(out: &mut String, numbers: &mut Vec<usize>, index: usize, limit: usize, remaining: i32) {
    if remaining == 0 {
        let result = numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
        writeln!(out, "{}", result).unwrap();
    } else {
        for i in index + 1..= limit {
            if numbers.contains(&i) {
                continue;
            }
            numbers.push(i);
            print_num(out, numbers, i, limit, remaining - 1);
            numbers.pop();
        }
    }
}

fn main() {
    let mut out = String::new();
    let args = read_line_as_numbers();
    let (n, m) = (args[0], args[1]);
    let mut numbers = Vec::new();
    print_num(&mut out, &mut numbers, 0, n as usize, m);
    print!("{}", out);
}
