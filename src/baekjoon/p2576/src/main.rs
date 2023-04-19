fn read_line_as_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap()
}

fn main() {
    let mut vec = Vec::new();
    for _ in 0..7 {
        let n = read_line_as_number();
        if n % 2 == 1 {
            vec.push(n);
        }
    }

    if vec.is_empty() {
        println!("-1");
    } else {
        println!("{}", vec.iter().sum::<u32>());
        println!("{}", vec.iter().min().unwrap());
    }
}
