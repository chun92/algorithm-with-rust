fn read_line_and_parse_plus_minus_and_numbers() -> (Vec<char>, Vec<i32>) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let str = line.trim();
    let nums = str
        .split(|c| c == '+' || c == '-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let op = str
        .chars()
        .filter(|c| c == &'+' || c == &'-')
        .collect::<Vec<char>>();
    (op, nums)
}

fn main() {
    let (op, nums) = read_line_and_parse_plus_minus_and_numbers();

    let mut sum = 0;
    let mut partial = 0;
    for i in 0..nums.len() {
        if i == 0 {
            sum += nums[i];
        } else {
            if op[i - 1] == '+' {
                if partial > 0 {
                    partial += nums[i];
                } else {
                    sum += nums[i];
                }
            } else if op[i - 1] == '-' {
                sum -= partial;
                partial = nums[i];
            }
        }
    }
    sum -= partial;
    println!("{}", sum);
}
