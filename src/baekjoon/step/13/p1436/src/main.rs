fn check_num_contains_666(num: u64) -> bool {
    num.to_string().contains("666")
}

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut count = 0;
    for i in 1.. {
        if check_num_contains_666(i) {
            count += 1;
            if count == n {
                println!("{}", i);
                break;
            }
        }
    }
}
