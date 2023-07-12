fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, s) = {
        let vec = read_line_as_numbers();
        (vec[0] as usize, vec[1])
    };
    let vec = read_line_as_numbers();
    let mut count = 0_u64;

    for i in 1..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & 1 << j != 0 {
                sum += vec[j];
            }
        }
        if sum == s {
            count += 1;
        }
    }

    println!("{}", count);
}
