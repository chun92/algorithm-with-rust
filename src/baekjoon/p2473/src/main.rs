fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_min_max(vec: &Vec<i64>, mid_index: usize, n: usize) -> (i64, i64, i64, i64) {
    let mut min = vec[0];
    let mid = vec[mid_index];
    let mut max = vec[n - 1];

    let mut start_index = 0;
    let mut end_index = n - 1;
    let mut min_sum = vec[start_index] + mid + vec[end_index];
    let mut cur_sum;
    while end_index > mid_index && start_index < mid_index {
        if min_sum == 0 {
            break;
        }
        cur_sum = vec[start_index] + mid + vec[end_index];
        if cur_sum.abs() < min_sum.abs() {
            min_sum = cur_sum;
            min = vec[start_index];
            max = vec[end_index];
        }

        if cur_sum > 0 {
            end_index -= 1;
        } else {
            start_index += 1;
        }
    }

    (min, mid, max, min_sum)
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let mut vec = read_line_as_numbers();

    vec.sort_unstable();
    let (mut min, mut mid, mut max, mut sum) = (0, 0, 0, i64::MAX);
    for i in 1..n - 1 {
        let (cur_min, cur_mid, cur_max, cur_sum) = get_min_max(&vec, i, n);
        if cur_sum == 0 {
            min = cur_min;
            mid = cur_mid;
            max = cur_max;
            break;
        } else if sum.abs() > cur_sum.abs() {
            min = cur_min;
            mid = cur_mid;
            max = cur_max;
            sum = cur_sum;
        }
    }

    println!("{} {} {}", min, mid, max);
}
