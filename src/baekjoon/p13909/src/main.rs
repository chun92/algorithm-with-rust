fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut cur = 1;
    loop {
        if cur * cur > n {
            break;
        }
        cur += 1;
    }
    println!("{}", cur - 1);
}