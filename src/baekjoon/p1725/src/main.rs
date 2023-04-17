fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn solve(vec: &Vec<u64>) -> u64 {
    solve_impl(&vec, (0, vec.len())).1
}

fn solve_impl(vec: &Vec<u64>, (left, right): (usize, usize)) -> ((usize, usize), u64) {
    if right - left == 0 {
        return ((left, right), 0);
    } else if right - left == 1 {
        let area = vec[left];
        return ((left , right), area);
    }

    let mid = (left + right) / 2;
    let left = solve_impl(vec, (left, mid));
    let right = solve_impl(vec, (mid, right));
    
    merge(vec, left, right)

}

fn merge(vec: &Vec<u64>, ((left_start, left_end), left_area): ((usize, usize), u64), ((right_start, right_end), right_area): ((usize, usize), u64)) -> ((usize, usize), u64) {
    let mut left_out_of_bound = false;
    let mut right_out_of_bound = false;
    let mut cur_left_iter = if left_start >= left_end {
        left_out_of_bound = true;
        left_end
    } else {
        left_end - 1
    };
    let mut cur_right_iter = if right_start >= right_end {
        right_out_of_bound = true;
        right_start
    } else {
        right_start
    };
    let mut cur_left = if left_out_of_bound {
        0
    } else {
        vec[cur_left_iter]
    };
    let mut cur_right = if right_out_of_bound {
        0
    } else {
        vec[cur_right_iter]
    };
    let mut cur_height = 
        if left_out_of_bound && right_out_of_bound {
            0
        } else if left_out_of_bound {
            cur_right
        } else if right_out_of_bound {
            cur_left
        } else {
            std::cmp::min(cur_left, cur_right)
        };
    let mut cur_width = 
        if left_out_of_bound && right_out_of_bound {
            0
        } else if left_out_of_bound {
            1
        } else if right_out_of_bound {
            1
        } else {
            2
        };
    let mut saved_area = cur_height * cur_width;

    while !left_out_of_bound || !right_out_of_bound {
        let new_left = if cur_left_iter > left_start {
            vec[cur_left_iter - 1]
        } else {
            left_out_of_bound = true;
            0
        };

        let new_right = if cur_right_iter < right_end - 1 {
            vec[cur_right_iter + 1]
        } else {
            right_out_of_bound = true;
            0
        };

        if left_out_of_bound && right_out_of_bound {
            break;
        }

        if new_left > new_right {
            if cur_left_iter > left_start {
                cur_left_iter -= 1;
                cur_width += 1;
                cur_left = new_left;
            } else {
                left_out_of_bound = true;
            }
        } else if new_left < new_right {
            if cur_right_iter < right_end - 1 {
                cur_right_iter += 1;
                cur_width += 1;
                cur_right = new_right;
            } else {
                right_out_of_bound = true;
            }
        } else {
            if cur_left_iter > left_start {
                cur_left_iter -= 1;
                cur_width += 1;
                cur_left = new_left;
            } else {
                left_out_of_bound = true;
            }
            if cur_right_iter < right_end - 1 {
                cur_right_iter += 1;
                cur_width += 1;
                cur_right = new_right;
            } else {
                right_out_of_bound = true;
            }
        }

        let new_height = std::cmp::min(cur_left, cur_right);
        
        cur_height = std::cmp::min(cur_height, new_height);
        let cur_area = cur_width * cur_height;
        saved_area = std::cmp::max(saved_area, cur_area);
    }

    let result_area = std::cmp::max(left_area, std::cmp::max(right_area, saved_area));

    ((left_start, right_end), result_area)
}

fn main() {
    let n = read_line_as_number();

    let mut numbers = Vec::new();
    for _ in 0..n {
        let number = read_line_as_number();
        numbers.push(number);
    }

    println!("{}", solve(&numbers));
}
