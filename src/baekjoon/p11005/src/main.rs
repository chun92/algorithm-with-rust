fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
fn main() {
    let (n, m) = {
        let i = read_line_as_numbers();
        (i[0], i[1])
    };

    if n == 0 {
        println!("0");
        return;
    }

    let mut temp = n;
    let mut result = String::new();

    while temp > 0 {
        let r = temp % m;
        let c = if r < 10 {
            (r + '0' as i32) as u8 as char
        } else {
            (r - 10 + 'A' as i32) as u8 as char
        };
        result.push(c);
        temp /= m;
    }

    println!("{}", result.chars().rev().collect::<String>());
}
