fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let nums = read_line_as_numbers();

    let mut sums = vec![0; n + 1];

    for i in 0..n {
        for j in (0..=i).rev() {
            if nums[i] > nums[j] {
                sums[i] = std::cmp::max(sums[i], sums[j] + nums[i]);
            }
        }

        if sums[i] == 0 {
            sums[i] = nums[i];
        }
    }

    println!("{}", sums.iter().max().unwrap());
}
