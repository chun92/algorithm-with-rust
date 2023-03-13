use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_max(nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    let mut map = HashMap::new();
    map.insert(0, nums[0]);
    let mut max = nums[0];

    for i in 1..n {
        let temp = std::cmp::max(map.get(&(i - 1)).unwrap() + nums[i], nums[i]);
        map.insert(i, temp);
        max = std::cmp::max(max, temp);
    }

    max
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let nums = read_line_as_numbers();
    println!("{}", get_max(&nums));
}