fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let args = read_line_as_strings();
    let n = args.get(0).unwrap();
    let m = args.get(1).unwrap().parse::<i32>().unwrap();

    let mut sum = 0;
    n
        .chars()
        .rev()
        .enumerate()
        .for_each(|(i, c)| {
            match c {
                ('0'..='9') => sum += c.to_digit(10).unwrap() as i32 * m.pow(i as u32),
                ('A'..='Z') => sum += (c as i32 - 55) * m.pow(i as u32),
                _ => panic!("Invalid input")
            }
        });
    println!("{}", sum);
}
