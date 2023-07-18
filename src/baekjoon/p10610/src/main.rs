fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();
    let mut nums =
        str.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
    
    let sum: u32 = nums
        .iter()
        .sum();

    if sum % 3 != 0 {
        println!("-1");
        return;
    }

    if nums.iter().all(|&n| n != 0) {
        println!("-1");
        return;
    }

    nums.sort_by(|a, b| b.cmp(a));
    nums
        .iter()
        .for_each(|n| print!("{}", n));
}
