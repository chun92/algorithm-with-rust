fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let m = read_line_as_number();
    let n = read_line_as_number();
    let mut min = i32::MAX;
    let mut sum = 0;
    for i in m..=n {
        let sqrt = (i as f64).sqrt() as i32;
        if sqrt * sqrt == i {
            min = min.min(i);
            sum += i;
        }
    }

    if sum == 0 {
        println!("-1");
    } else {
        println!("{}", sum);
        println!("{}", min);
    }
}
