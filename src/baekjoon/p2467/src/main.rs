fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let vec = read_line_as_numbers();
    let mut min = vec[0];
    let mut max = vec[n - 1];

    let mut start_index = 0;
    let mut end_index = n - 1;
    let mut min_sum = vec[start_index] + vec[end_index];
    let mut cur_sum;
    while end_index > start_index {
        if min_sum == 0 {
            break;
        }
        cur_sum = vec[start_index] + vec[end_index];
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

    println!("{} {}", min, max);
}
