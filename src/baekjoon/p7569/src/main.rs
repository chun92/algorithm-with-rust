use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn check_all_ripe(vec: &Vec<Vec<Vec<i32>>>) -> bool {
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            for k in 0..vec[i][j].len() {
                if vec[i][j][k] == 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let (n, m, h) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1] as usize, v[2] as usize)
    };

    let mut vec = vec![vec![vec![0; n]; m]; h];
    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..m {
            let v = read_line_as_numbers();
            for k in 0..n {
                vec[i][j][k] = v[k];
                if v[k] == 1 {
                    queue.push_back((i, j, k));
                }
            }
        }
    }

    let mut count = 0;
    if check_all_ripe(&vec) {
        println!("{}", count);
        return;
    }

    while !queue.is_empty() {
        let mut new_queue = VecDeque::new();
        while !queue.is_empty() {
            let (i, j, k) = queue.pop_front().unwrap();
            if i > 0 && vec[i - 1][j][k] == 0 {
                vec[i - 1][j][k] = 1;
                new_queue.push_back((i - 1, j, k));
            }
            if i < h - 1 && vec[i + 1][j][k] == 0 {
                vec[i + 1][j][k] = 1;
                new_queue.push_back((i + 1, j, k));
            }
            if j > 0 && vec[i][j - 1][k] == 0 {
                vec[i][j - 1][k] = 1;
                new_queue.push_back((i, j - 1, k));
            }
            if j < m - 1 && vec[i][j + 1][k] == 0 {
                vec[i][j + 1][k] = 1;
                new_queue.push_back((i, j + 1, k));
            }
            if k > 0 && vec[i][j][k - 1] == 0 {
                vec[i][j][k - 1] = 1;
                new_queue.push_back((i, j, k - 1));
            }
            if k < n - 1 && vec[i][j][k + 1] == 0 {
                vec[i][j][k + 1] = 1;
                new_queue.push_back((i, j, k + 1));
            }
        }

        if new_queue.is_empty() {
            break;
        } else {
            queue = new_queue;
        }
        count += 1;
    }
    
    if check_all_ripe(&vec) {
        println!("{}", count);
    } else {
        println!("{}", -1);
    }
}
