fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut zero_count = 0;
    let mut one_count = 0;
    for _ in 0..n {
        let input = read_line_as_number();
        if input == 0 {
            zero_count += 1;
        } else {
            one_count += 1;
        }
    }

    if zero_count > one_count {
        println!("Junhee is not cute!");
    } else if zero_count < one_count {
        println!("Junhee is cute!");
    }
}
