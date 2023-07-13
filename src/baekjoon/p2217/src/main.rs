fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut vec = Vec::new();

    for _ in 0..n {
        let x = read_line_as_number();
        vec.push(x);
    }

    vec.sort_unstable();
    let mut max = 0;

    for i in 0..n {
        let sum = vec[i] * (n - i);
        if sum > max {
            max = sum;
        }
    }

    println!("{}", max);
}