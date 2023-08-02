fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let l = read_line_as_number();
    let a = read_line_as_number();
    let b = read_line_as_number();
    let c = read_line_as_number();
    let d = read_line_as_number();

    let korean = (a as f64 / c as f64).ceil() as usize;
    let math = (b as f64 / d as f64).ceil() as usize;
    let max = std::cmp::max(korean, math);
    println!("{}", l - max);
}
