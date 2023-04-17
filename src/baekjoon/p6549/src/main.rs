fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
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

        // println!("left_iter: {}/{} right_iter: {}/{} cur_left: {}, cur_right: {}, cur_height: {}, cur_width: {}, cur_area: {}, saved_area: {}", cur_left_iter, left_start, cur_right_iter, right_end, cur_left, cur_right, cur_height, cur_width, cur_area, saved_area);
    }
    let result_area = std::cmp::max(left_area, std::cmp::max(right_area, saved_area));

    ((left_start, right_end), result_area)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_1() {
        let vec = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        let (result_index, result_area) = merge(&vec, ((0, 5), 9), ((5, 10), 9));
        assert_eq!(result_index, (0, 10));
        assert_eq!(result_area, 10);
    }

    #[test]
    fn test_merge_2() {
        let vec = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        let (result_index, result_area) = merge(&vec, ((0, 5), 9), ((5, 10), 9));
        assert_eq!(result_index, (0, 10));
        assert_eq!(result_area, 18);
    }
    
    #[test]
    fn test_merge_3() {        
        let vec = vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
        let (result_index, result_area) = merge(&vec, ((0, 5), 25), ((5, 10), 25));
        assert_eq!(result_index, (0, 10));
        assert_eq!(result_area, 50);
    }
    
    #[test]
    fn test_merge_4() {
        let vec = vec![7, 2, 1, 4, 5, 1, 3, 3];
        let (result_index, result_area) = merge(&vec, ((0, 4), 7), ((4, 8), 6));
        assert_eq!(result_index, (0, 8));
        assert_eq!(result_area, 8);
    }

    #[test]
    fn test_merge_5() {
        let vec = vec![1, 2, 3, 4, 5];
        let (result_index, result_area) = merge(&vec, ((0, 5), 9), ((5, 5), 0));
        assert_eq!(result_index, (0, 5));
        assert_eq!(result_area, 9);
    }

    #[test]
    fn test_merge_6() {
        let vec = vec![1, 2, 3, 4, 5];
        let (result_index, result_area) = merge(&vec, ((0, 0), 0), ((0, 5), 9));
        assert_eq!(result_index, (0, 5));
        assert_eq!(result_area, 9);
    }

    #[test]
    fn test_solve_1() {
        let vec = vec![7, 2, 1, 4, 5, 1, 3, 3];
        let result = solve(&vec);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_solve_2() {
        let vec = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        let result = solve(&vec);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_solve_3() {
        let vec = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        let result = solve(&vec);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_solve_4() {
        let vec = vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
        let result = solve(&vec);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_solve_5() {
        let vec = vec![4, 1000, 1000, 1000, 1000];
        let result = solve(&vec);
        assert_eq!(result, 4000);
    }
    
    #[test]
    fn test_solve_6() {
        let vec = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        let result = solve(&vec);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_solve_7() {
        let vec = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
        let result = solve(&vec);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_solve_8() {
        let vec = vec![1];
        let result = solve(&vec);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solve_9() {
        let vec = vec![1, 2, 2, 2, 2, 2, 10, 10, 2, 2, 2, 2, 2, 1];
        let result = solve(&vec);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_solve_10() {
        let vec = vec![5, 5, 10, 5, 11, 10];
        let result = solve(&vec);
        assert_eq!(result, 30);
    }
}

fn main() {
    loop {
        let numbers = read_line_as_numbers();
        if numbers.len() == 1 && numbers[0] == 0 {
            break;
        }

        println!("{}", solve(&numbers[1..].to_vec()));
    }
}
