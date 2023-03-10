fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn main() {
    let k = read_line_as_number();

    let mut vec = Vec::new();
    for _ in 0..k {
        let n = read_line_as_number();
        match n {
            0 => { vec.pop(); },
            _ => { vec.push(n); }
        }
    }

    println!("{}", vec.iter().sum::<u64>());
}
