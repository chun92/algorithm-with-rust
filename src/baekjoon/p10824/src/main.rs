fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let strs = read_line_as_strings();
    let s1 = String::from(&strs[0]) + &strs[1];
    let s2 = String::from(&strs[2]) + &strs[3];
    let sum = s1.parse::<u64>().unwrap() + s2.parse::<u64>().unwrap();
    println!("{}", sum);
}
