fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let n = read_line_as_number();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(read_line_as_number());
    }
    v.sort_by(|a, b| b.cmp(a));
    
    let mut diff = Vec::new();
    let mut prev = 0;
    for (i, &v) in v.iter().enumerate() {
        if i == 0 {    
            prev = v;
            continue;
        }
        diff.push(prev - v);
        prev = v;
    }

    let gcd = diff
        .iter()
        .fold(diff[0], |acc, &x| gcd(acc, x));

    let result = diff
        .iter()
        .fold(0, |acc, &x| acc + x / gcd - 1);
    
    println!("{}", result);
}
