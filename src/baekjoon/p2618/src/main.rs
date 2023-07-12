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

// another answer with bottom up m
fn get_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let (ax, ay) = a;
    let (bx, by) = b;
    ax.abs_diff(*bx) + ay.abs_diff(*by)
}

fn answer2() {
    let n = read_line_as_numbers()[0];
    let w = read_line_as_numbers()[0] as usize;
    let mut vec = Vec::new();

    vec.push((0, 0));
    for _ in 0..w {
        let pair = read_line_as_numbers();
        vec.push((pair[0], pair[1]));
    }

    let mut memo = vec![vec![usize::MAX; w + 1]; w + 1];
    let mut backword_memo = vec![vec![(0, 0); w + 1]; w + 1];
    let a0_position = (1, 1);
    let b0_position = (n, n);

    memo[1][0] = get_distance(&a0_position, &vec[1]);
    backword_memo[1][0] = (0, 0);
    memo[0][1] = get_distance(&vec[1], &b0_position);
    backword_memo[0][1] = (0, 0);

    for i in 1..w {
        for j in 0..i {
            let a_position = vec[i];
            let b_position = if j == 0 { b0_position } else { vec[j] };
            let next_position = vec[i + 1];

            let current_distance = memo[i][j];
            let tareget_distance = memo[i + 1][j];
            let next_distance = current_distance + get_distance(&a_position, &next_position);
            if next_distance < tareget_distance {
                memo[i + 1][j] = next_distance;
                backword_memo[i + 1][j] = (i, j);
            }

            let target_distance = memo[i][i + 1];
            let next_distance = current_distance + get_distance(&b_position, &next_position);
            if next_distance < target_distance {
                memo[i][i + 1] = next_distance;
                backword_memo[i][i + 1] = (i, j);
            }
        }

        for j in 0..i {
            let a_position = if j == 0 { a0_position } else { vec[j] };
            let b_position = vec[i];
            let next_position = vec[i + 1];

            let current_distance = memo[j][i];
            let target_distance = memo[j][i + 1];
            let next_distance = current_distance + get_distance(&b_position, &next_position);

            if next_distance < target_distance {
                memo[j][i + 1] = next_distance;
                backword_memo[j][i + 1] = (j, i);
            }

            let target_distance = memo[i + 1][i];
            let next_distance = current_distance + get_distance(&a_position, &next_position);
            if next_distance < target_distance {
                memo[i + 1][i] = next_distance;
                backword_memo[i + 1][i] = (j, i);
            }
        }
    }

    let mut answer = usize::MAX;
    let mut current = (w, w);
    for i in 0..w {
        if answer > memo[w][i] {
            answer = memo[w][i];
            current = (w, i);
        }
        if answer > memo[i][w] {
            answer = memo[i][w];
            current = (i, w);
        }
    }

    let mut path = Vec::new();
    while current != (0, 0) {
        let prev = backword_memo[current.0][current.1];
        if current.0 == prev.0 {
            path.push(2);
        } else {
            path.push(1);
        }
        current = prev;
    }

    println!("{}", answer);
    for i in (0..path.len()).rev() {
        println!("{}", path[i]);
    }
}