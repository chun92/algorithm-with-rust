fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    loop {
        let number = read_line_as_number();
        if number == 0 {
            break;
        }

        let result = number * (number + 1) / 2;
        println!("{}", result);
    }
}
