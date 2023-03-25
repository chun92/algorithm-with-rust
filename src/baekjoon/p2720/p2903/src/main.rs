fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();

    let res = 2_i32.pow(n as u32) + 1;
    println!("{}", res.pow(2));
}
