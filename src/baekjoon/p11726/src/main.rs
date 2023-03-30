fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();

    let mut vec = vec![0; n + 1];

    vec[0] = 1;
    vec[1] = 1;

    for i in 2..=n {
        vec[i] = (vec[i - 1] + vec[i - 2]) % 10007;
    }

    println!("{}", vec[n]);
}
