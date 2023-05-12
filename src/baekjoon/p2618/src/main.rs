fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(memo: &mut Vec<Vec<usize>>, vec: &Vec<(usize, usize)>, i: usize, j: usize, a_position: (usize, usize), b_position: (usize, usize)) -> usize {
    if i == vec.len() - 1 || j == vec.len() - 1 {
        return 0;
    }

    if memo[i][j] > 0 {
        return memo[i][j];
    }

    let index = std::cmp::max(i, j) + 1;
    
    let dist_a = a_position.0.abs_diff(vec[index].0) + a_position.1.abs_diff(vec[index].1);
    let dist_b = b_position.0.abs_diff(vec[index].0) + b_position.1.abs_diff(vec[index].1);
    let dist_a = dist_a + dp(memo, vec, index, j, (vec[index].0, vec[index].1), b_position);
    let dist_b = dist_b + dp(memo, vec, i, index, a_position, (vec[index].0, vec[index].1));

    let result = std::cmp::min(dist_a, dist_b);
    memo[i][j] = result;
    
    result
}

fn get_path(memo: &mut Vec<Vec<usize>>, vec: &Vec<(usize, usize)>, i: usize, j: usize, a_position: (usize, usize), b_position: (usize, usize)) {
    if i == vec.len() - 1 || j == vec.len() - 1 {
        return;
    }

    let index = std::cmp::max(i, j) + 1;
    
    let dist_a = a_position.0.abs_diff(vec[index].0) + a_position.1.abs_diff(vec[index].1);
    let dist_b = b_position.0.abs_diff(vec[index].0) + b_position.1.abs_diff(vec[index].1);
    let dist_a = dist_a + dp(memo, vec, index, j, (vec[index].0, vec[index].1), b_position);
    let dist_b = dist_b + dp(memo, vec, i, index, a_position, (vec[index].0, vec[index].1));

    if dist_a < dist_b {
        println!("1");
        get_path(memo, vec, index, j, (vec[index].0, vec[index].1), b_position);
    } else {
        println!("2");
        get_path(memo, vec, i, index, a_position, (vec[index].0, vec[index].1));
    }
}

fn main() {
    let n = read_line_as_numbers()[0];
    let w = read_line_as_numbers()[0];
    let mut vec = Vec::new();

    vec.push((0, 0));
    for _ in 0..w {
        let pair = read_line_as_numbers();
        vec.push((pair[0], pair[1]));
    }

    let mut memo = vec![vec![0; w + 1]; w + 1];
    let a_position = (1, 1);
    let b_position = (n, n);
    let result = dp(&mut memo, &vec, 0, 0, a_position, b_position);
    println!("{}", result);

    get_path(&mut memo, &vec, 0, 0, a_position, b_position);
}