fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

fn get_lcm(a: u32, b: u32) -> u32 {
    a * b / get_gcd(a, b)
}

fn main() {
    let nums = read_line_as_numbers();
    let mut result = u32::MAX;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                result = std::cmp::min(result, get_lcm(nums[i], get_lcm(nums[j], nums[k])));
            }
        }
    }
    println!("{}", result);
}
