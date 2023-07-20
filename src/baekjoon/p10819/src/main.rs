fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(max: &mut i32, vec: &Vec<i32>, remain_index_vec: &Vec<usize>, current_index_vec: &Vec<usize>) {
    if remain_index_vec.len() == 0 {
        let mut sum = 0;
        for i in 0..current_index_vec.len() - 1 {
            sum += (vec[current_index_vec[i]] - vec[current_index_vec[i + 1]]).abs();
        }
        if sum > *max {
            *max = sum;
        }
        return;
    }

    for i in 0..remain_index_vec.len() {
        let mut current_index_vec_clone = current_index_vec.clone();
        current_index_vec_clone.push(remain_index_vec[i]);
        let mut remain_index_vec_clone = remain_index_vec.clone();
        remain_index_vec_clone.remove(i);
        dp(max, vec, &remain_index_vec_clone, &current_index_vec_clone);
    }
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let vec = read_line_as_numbers();
    let index_vec = (0..n).collect::<Vec<usize>>();
    let mut max = 0;
    dp(&mut max, &vec, &index_vec, &vec![]);
    println!("{}", max);
}
