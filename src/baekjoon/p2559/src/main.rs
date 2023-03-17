fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_sums() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut sums = Vec::new();
    let mut sum = 0;
    for s in line.split_whitespace() {
        sum += s.parse::<i32>().unwrap();
        sums.push(sum);
    }
    sums
}

fn get_max_with_interval(sums: &Vec<i32>, m: i32) -> i32 {
    let mut max = i32::min_value();
    sums
        .iter()
        .enumerate()
        .for_each(|(i, s)| {
            match i {
                x if x < (m - 1) as usize => return,
                x if x == (m - 1) as usize => {
                    max = *s
                },
                _ => {
                    let target = sums[i] - sums[i - m as usize];
                    if target > max {
                        max = target;
                    }
                }
            }
        });
    max
}

fn main() {
    let args = read_line_as_numbers();
    let (_n, m) = (args[0], args[1]);
    let sums = read_line_as_sums();
    println!("{}", get_max_with_interval(&sums, m));
}
