fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_sum_of_num(nums: &Vec<u64>, cur: u64) -> u64 {
    nums
        .iter()
        .fold(0, |acc, &x| if x > cur { acc + x - cur } else { acc })
}

fn calc_max(nums: &Vec<u64>, m: u64, cur: &mut u64, min: &mut u64, max: &mut u64) {
    if min > max {
        return;
    }

    let mid = (*min + *max) / 2;
    let sum = get_sum_of_num(nums, mid);
    if sum < m {
        *max = mid - 1;
        calc_max(nums, m, cur, min, max)
    } else {
        *cur = mid;
        *min = mid + 1;
        calc_max(nums, m, cur, min, max)
    }
}

fn main() {
    let numbers = read_line_as_numbers();
    let (_n, m) = (numbers[0], numbers[1]);
    let nums = read_line_as_numbers();

    let mut max = u64::MAX;
    let mut min = u64::MIN;
    let mut cur = 0;
    calc_max(&nums, m, &mut cur, &mut min, &mut max);
    println!("{}", cur);
}
