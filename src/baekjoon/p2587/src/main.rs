fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut numbers = Vec::<i32>::new();
    for _ in 0..5 {
        numbers.push(read_line_as_number());
    }

    let mut sum = 0;
    let mut mid = 0;

    numbers.sort();
    numbers
        .iter()
        .enumerate()
        .for_each(|(i, n)| {
            sum += *n;
            if i == 2 {
                mid = *n;
            }
        });
    println!("{}", sum / 5);
    println!("{}", mid);
}
