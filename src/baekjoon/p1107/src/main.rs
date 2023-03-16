fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = solve(5457, 3, vec![6, 7, 8]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = solve(100, 5, vec![0, 1, 2, 3, 4]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let result = solve(500000, 8, vec![0, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(result, 11117);
    }

    #[test]
    fn test4() {
        let result = solve(100, 3, vec![1, 0, 5]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = solve(14124, 0, vec![]);
        assert_eq!(result, 5);
    }

    #[test]
    fn test6() {
        let result = solve(1, 9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test7() {
        let result = solve(80000, 2, vec![8, 9]);
        assert_eq!(result, 2228);
    }

    #[test]
    fn test_advanced1() {
        let result = solve(10, 9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_advanced2() {
        let result = solve(101, 0, vec![]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_advanced3() {
        let result = solve(99, 10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_advanced4() {
        let result = solve(0, 0, vec![]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_advanced5() {
        let result = solve(500000, 6, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(result, 166672);
    }

    #[test]
    fn test_advanced6() {
        let result = solve(0, 2, vec![0, 1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_advanced7() {
        let result = solve(1555, 8, vec![0, 1, 3, 4, 5, 6, 7, 9]);
        assert_eq!(result, 670);
    }

    #[test]
    fn test_advanced8() {
        let result = solve(944, 7, vec![2, 3, 4, 5, 6, 7, 9]);
        assert_eq!(result, 59);
    }

//     6 
// 9
// 0 2 3 4 5 6 7 8 9
// 답:6

    #[test]
    fn test_advanced9() {
        let result = solve(6, 9, vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 6);
    }

    // 500000
    // 10
    // 0 1 2 3 4 5 6 7 8 9
    // 답:499900
    
    #[test]
    fn test_advanced10() {
        let result = solve(500000, 10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 499900);
    }
    
    // 101 
    // 10
    // 0 1 2 3 4 5 6 7 8 9
    // 답:1

    #[test]
    fn test_advanced11() {
        let result = solve(101, 10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 1);
    }
    
    // 1
    // 9
    // 1 2 3 4 5 6 7 8 9
    // 답:2

    #[test]
    fn test_advanced12() {
        let result = solve(1, 9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 2);
    }
    
    // 99999
    // 1
    // 9
    // 답:7

    #[test]
    fn test_advanced13() {
        let result = solve(99999, 1, vec![9]);
        assert_eq!(result, 7);
    }
    
    // 10
    // 1
    // 0
    // 답:2

    #[test]
    fn test_advanced14() {
        let result = solve(10, 1, vec![0]);
        assert_eq!(result, 2);
    }
    
    // 0
    // 3
    // 0 1 2
    // 답:4

    #[test]
    fn test_advanced15() {
        let result = solve(0, 3, vec![0, 1, 2]);
        assert_eq!(result, 4);
    }
    
    // 0
    // 9
    // 0 1 2 3 4 5 6 7 8
    // 답:10

    #[test]
    fn test_advanced16() {
        let result = solve(0, 9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 10);
    }
    
    // 0
    // 10
    // 0 1 2 3 4 5 6 7 8 9
    // 답:100

    #[test]
    fn test_advanced17() {
        let result = solve(0, 10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 100);
    }
    
    // 1
    // 9
    // 1 2 3 4 5 6 7 8 9
    // 답:2

    #[test]
    fn test_advanced18() {
        let result = solve(1, 9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 2);
    }
    
    // 1020
    // 0
    // 답:4

    #[test]
    fn test_advanced19() {
        let result = solve(1020, 0, vec![]);
        assert_eq!(result, 4);
    }
    
    // 10 
    // 2
    // 0 1
    // 답:2

    #[test]
    fn test_advanced20() {
        let result = solve(10, 2, vec![0, 1]);
        assert_eq!(result, 2);
    }
    
    // 999
    // 1
    // 9
    // 답:5

    #[test]
    fn test_advanced21() {
        let result = solve(999, 1, vec![9]);
        assert_eq!(result, 5);
    }
    
    // 9990
    // 8
    // 1 2 3 4 5 6 7 8
    // 답:4

    #[test]
    fn test_advanced22() {
        let result = solve(9990, 8, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 4);
    }
    
    // 123
    // 2
    // 2 3
    // 답:7

    #[test]
    fn test_advanced23() {
        let result = solve(123, 2, vec![2, 3]);
        assert_eq!(result, 7);
    }
    
    // 199
    // 1 
    // 9
    // 답:4

    #[test]
    fn test_advanced24() {
        let result = solve(199, 1, vec![9]);
        assert_eq!(result, 4);
    }
    
    // 9
    // 5
    // 9 8 7 6 5
    // 답:3

    #[test]
    fn test_advanced25() {
        let result = solve(9, 5, vec![9, 8, 7, 6, 5]);
        assert_eq!(result, 3);
    }
    
    // 19
    // 1
    // 1
    // 답:3

    #[test]
    fn test_advanced26() {
        let result = solve(19, 1, vec![1]);
        assert_eq!(result, 3);
    }

    //     5959
    // 4
    // 1 2 3 4 
    // 4
    // -----------------------------------------------

    #[test]
    fn test_another1() {
        let result = solve(5959, 4, vec![1, 2, 3, 4]);
        assert_eq!(result, 4);
    }
    // 56666
    // 0

    // 5
    // -----------------------------------------------
    #[test]
    fn test_another2() {
        let result = solve(56666, 0, vec![]);
        assert_eq!(result, 5);
    }

    // 9999
    // 8
    // 0 1 2 3 4 5 6 7 
    // 4
    // -----------------------------------------------
    #[test]
    fn test_another3() {
        let result = solve(9999, 8, vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result, 4);
    }

    // 10
    // 1
    // 1 
    // 2
    // -----------------------------------------------
    #[test]
    fn test_another4() {
        let result = solve(10, 1, vec![1]);
        assert_eq!(result, 2);
    }

    // 190000
    // 3
    // 1 2 9 
    // 101117
    // -----------------------------------------------
    #[test]
    fn test_another5() {
        let result = solve(190000, 3, vec![1, 2, 9]);
        assert_eq!(result, 101117);
    }

    // 123
    // 3
    // 1 2 5 
    // 23
    // -----------------------------------------------
    #[test]
    fn test_another6() {
        let result = solve(123, 3, vec![1, 2, 5]);
        assert_eq!(result, 23);
    }

    // 1
    // 9
    // 0 1 2 3 4 5 6 7 8 
    // 9
    // -----------------------------------------------
    #[test]
    fn test_another7() {
        let result = solve(1, 9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 9);
    }

    // 100
    // 10
    // 0 1 2 3 4 5 6 7 8 9 
    // 0
    // -----------------------------------------------
    #[test]
    fn test_another8() {
        let result = solve(100, 10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 0);
    }

    // 99933
    // 2
    // 3 9 
    // 73
    // -----------------------------------------------
    #[test]
    fn test_another9() {
        let result = solve(99933, 2, vec![3, 9]);
        assert_eq!(result, 73);
    }

    // 1023
    // 5
    // 1 2 3 4 0 
    // 27
    // -----------------------------------------------
    #[test]
    fn test_another10() {
        let result = solve(1023, 5, vec![1, 2, 3, 4, 0]);
        assert_eq!(result, 27);
    }

    // 91010
    // 2
    // 1 0 
    // 1016
    // -----------------------------------------------
    #[test]
    fn test_another11() {
        let result = solve(91010, 2, vec![1, 0]);
        assert_eq!(result, 1016);
    }

    // 383399
    // 6
    // 1 2 3 4 5 7 
    // 216607
    // -----------------------------------------------
    #[test]
    fn test_another12() {
        let result = solve(383399, 6, vec![1, 2, 3, 4, 5, 7]);
        assert_eq!(result, 216607);
    }

    // 6711
    // 2
    // 1 2 
    // 6
    // -----------------------------------------------
    #[test]
    fn test_another13() {
        let result = solve(6711, 2, vec![1, 2]);
        assert_eq!(result, 6);
    }

    // 330
    // 4
    // 0 1 2 3 
    // 117
    // -----------------------------------------------
    #[test]
    fn test_another14() {
        let result = solve(330, 4, vec![0, 1, 2, 3]);
        assert_eq!(result, 117);
    }

    // 71923
    // 5
    // 4 5 6 7 9 
    // 8082
    // -----------------------------------------------
    #[test]
    fn test_another15() {
        let result = solve(71923, 5, vec![4, 5, 6, 7, 9]);
        assert_eq!(result, 8082);
    }

    // 123123
    // 3
    // 1 2 3 
    // 23129
    // -----------------------------------------------
    #[test]
    fn test_another16() {
        let result = solve(123123, 3, vec![1, 2, 3]);
        assert_eq!(result, 23129);
    }

    // 499999
    // 2
    // 4 8 
    // 7
    // -----------------------------------------------
    #[test]
    fn test_another17() {
        let result = solve(499999, 2, vec![4, 8]);
        assert_eq!(result, 7);
    }

    // 1111
    // 9
    // 1 2 3 4 5 6 7 8 9 
    // 1011
    // -----------------------------------------------
    #[test]
    fn test_another18() {
        let result = solve(1111, 9, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 1011);
    }

    // 1111
    // 9
    // 0 1 2 3 4 5 6 7 8 
    // 115
    // -----------------------------------------------
    #[test]
    fn test_another19() {
        let result = solve(1111, 9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 115);
    }

    // 34311
    // 8
    // 0 1 2 3 4 5 6 7 
    // 24316
    // -----------------------------------------------
    #[test]
    fn test_another20() {
        let result = solve(34311, 8, vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result, 24316);
    }


    // 49445
    // 7
    // 1 2 3 4 5 6 7 
    // 30560
    // -----------------------------------------------
    #[test]
    fn test_another21() {
        let result = solve(49445, 7, vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result, 30560);
    }

    // 933
    // 2
    // 1 2 
    // 3
    // -----------------------------------------------
    #[test]
    fn test_another22() {
        let result = solve(933, 2, vec![1, 2]);
        assert_eq!(result, 3);
    }

    // 1617
    // 3
    // 1 2 3 
    // 621
    // -----------------------------------------------
    #[test]
    fn test_another23() {
        let result = solve(1617, 3, vec![1, 2, 3]);
        assert_eq!(result, 621);
    }

    // 856
    // 2
    // 5 6 
    // 10
    // -----------------------------------------------
    #[test]
    fn test_another24() {
        let result = solve(856, 2, vec![5, 6]);
        assert_eq!(result, 10);
    }

    // 1023
    // 8
    // 1 2 3 4 5 6 7 8 
    // 27
    // -----------------------------------------------
    #[test]
    fn test_another25() {
        let result = solve(1023, 8, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 27);
    }

    // 10900
    // 2
    // 1 0 
    // 905
    // -----------------------------------------------
    #[test]
    fn test_another26() {
        let result = solve(10900, 2, vec![1, 0]);
        assert_eq!(result, 905);
    }

    // 394344
    // 3
    // 1 2 3 
    // 5662
    // -----------------------------------------------
    #[test]
    fn test_another27() {
        let result = solve(394344, 3, vec![1, 2, 3]);
        assert_eq!(result, 5662);
    }

    // 99
    // 1
    // 8 
    // 1
    // -----------------------------------------------
    #[test]
    fn test_another28() {
        let result = solve(99, 1, vec![8]);
        assert_eq!(result, 1);
    }

    // 101
    // 9
    // 0 1 2 3 4 5 6 7 8 
    // 1
    // -----------------------------------------------
    #[test]
    fn test_another29() {
        let result = solve(101, 9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 1);
    }

    // 2420
    // 6
    // 1 2 3 4 5 6 
    // 1424
    // -----------------------------------------------
    #[test]
    fn test_another30() {
        let result = solve(2420, 6, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(result, 1424);
    }

    // 991
    // 1
    // 1 
    // 4
    // -----------------------------------------------
    #[test]
    fn test_another31() {
        let result = solve(991, 1, vec![1]);
        assert_eq!(result, 4);
    }

    // 30002
    // 3
    // 1 3 4 
    // 8
    // -----------------------------------------------
    #[test]
    fn test_another32() {
        let result = solve(30002, 3, vec![1, 3, 4]);
        assert_eq!(result, 8);
    }

    // 1698
    // 2
    // 6 9 
    // 6
    // -----------------------------------------------
    #[test]
    fn test_another33() {
        let result = solve(1698, 2, vec![6, 9]);
        assert_eq!(result, 6);
    }

    // 499998
    // 3
    // 4 8 9 
    // 8
    // -----------------------------------------------
    #[test]
    fn test_another34() {
        let result = solve(499998, 3, vec![4, 8, 9]);
        assert_eq!(result, 8);
    }

    // 1022
    // 5
    // 1 2 3 4 5 
    // 26
    // -----------------------------------------------
    #[test]
    fn test_another35() {
        let result = solve(1022, 5, vec![1, 2, 3, 4, 5]);
        assert_eq!(result, 26);
    }

    // 5

    // 2
    
    // 4 6
    
    // 답 : 1
    #[test]
    fn test_extra1() {
        let result = solve(5, 2, vec![4, 6]);
        assert_eq!(result, 1);
    }
//     889
// 9
// 0 2 3 4 5 6 7 8 9
// 답: 226
    #[test]
    fn test_extra2() {
        let result = solve(889, 9, vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result, 226);
    }
}

fn get_max_num(target: &i32, nums: &Vec<i32>, n: i32, carry: bool) -> i32 {
    if n == 0 {
        return 0;
    }

    let zeros = 10i32.pow((n - 1) as u32);
    let target_digit = (target / zeros) % 10;
    let new_target = target - target_digit * zeros;

    if carry {
        let digit = nums.iter().min().unwrap();
        let remain = get_max_num(&new_target, nums, n - 1, true);
        if remain == -1 {
            return - 1;
        }
        let result = digit * zeros + get_max_num(&new_target, nums, n - 1, true);
        if !get_exact_num(&result, nums) {
            return -1;
        }
        return result;
    }

    for i in nums {
        match *i {
            x if x == target_digit => {
                let remain = get_max_num(&new_target, nums, n - 1, false);
                if remain == -1 {
                    continue;
                }
                let result = x * zeros + remain;
                if !get_exact_num(&result, nums) {
                    continue;
                }
                if result > *target {
                    return result;
                }
            },
            x if x > target_digit => {
                let remain = get_max_num(&new_target, nums, n - 1, true);
                if remain == -1 {
                    continue;
                }
                let result = x * zeros + remain;
                if !get_exact_num(&result, nums) {
                    continue;
                }
                return result;
            }, 
            _ => {
                continue;
            }
        }
    }

    return -1;
}


fn get_min_num(target: &i32, nums: &Vec<i32>, n: i32, carry: bool, is_first_digit: bool) -> i32 {
    // println!("target: {}, n: {}, carry: {}, is_first_digit: {}", target, n, carry, is_first_digit);
    if n == 1 {
        if carry {
            let digit = nums.iter().max().unwrap();
            // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, *digit, *digit);
            return *digit;
        } else {
            for i in nums.iter().rev() {
                if i <= target {
                    // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, *i, *i);
                    return *i;
                }
            }
            // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, -1, -1);
            return -1;
        }
    }

    let zeros = 10i32.pow((n - 1) as u32);
    let target_digit = (target / zeros) % 10;
    let new_target = target - target_digit * zeros;

    if carry {
        let digit = nums.iter().max().unwrap();
        let remain = get_min_num(&new_target, nums, n - 1, true, false);
        if remain == -1 {
            return - 1;
        }
        let result = digit * zeros + remain;
        if !get_exact_num(&result, nums) {
            return -1;
        }
        // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, digit, result);
        return result;
    }

    for i in nums.iter().rev() {
        match *i {
            x if x == target_digit => {
                let remain = get_min_num(&new_target, nums, n - 1, false, false);

                if remain == -1 {
                    continue;
                }
                let result = x * zeros + remain;
                if !get_exact_num(&result, nums) {
                    continue;
                }
                if result < *target {
                    // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, x, result);
                    return result;
                }
            },
            x if x < target_digit => {
                if is_first_digit && x == 0 {
                    return get_min_num(&new_target, nums, n - 1, true, false);
                }
                let remain = get_min_num(&new_target, nums, n - 1, true, false);

                if remain == -1 {
                   continue;
                }
                let result = x * zeros + remain;
                if !get_exact_num(&result, nums) {
                    continue;
                }
                // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, x, result);
                return result;
            }, 
            _ => {
                continue;
            }
        }
    }

    // println!("target: {}, n: {}, carry: {}, is_first_digit: {}, digit: {}, result: {}", target, n, carry, is_first_digit, -1, -1);
    return get_min_num(&new_target, nums, n - 1, true, false);
}

fn get_digit_num(num: &i32) -> i32 {
    if *num == 0 {
        return 1;
    }
    let mut n = *num;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    return count;
}

fn get_exact_num(target: &i32, nums: &Vec<i32>) -> bool {
    let mut result = true;
    target
        .to_string()
        .chars()
        .for_each(|x| {
            if !nums.contains(&(x.to_digit(10).unwrap() as i32)) {
                result = false;
            }
        });
    return result;
}

fn get_clicked_button_num(target: &i32, n: &i32) -> i32 {
    let x = get_digit_num(n);
    return (target - n).abs() + x;
}

fn solve(target: i32, n: i32, removing: Vec<i32>) -> i32 {
    if n == 0 {
        let result = (target - 100).abs();
        let click_all = get_clicked_button_num(&target, &target);
        let result = std::cmp::min(result, click_all);
        return result;
    }
    
    let mut nums = (0..=9).collect::<Vec<i32>>();

    if n == 10 {
        return (target - 100).abs();
    }

    for r in removing {
        nums.retain(|&n| n != r);
    }
    
    if get_exact_num(&target, &nums) {
        return std::cmp::min(get_clicked_button_num(&target, &target), (target - 100).abs());
    }

    let digit_num = get_digit_num(&target);
    let mut max = get_max_num(&target, &nums, digit_num, false);
    let min = get_min_num(&target, &nums, digit_num, false, true);

    if max == -1 {
        let zeros = 10i32.pow(digit_num as u32);
        if get_exact_num(&zeros, &nums) {
            max = zeros;
        } else if nums.contains(&0) && nums.len() == 1 {
            max = -1;
        } else {
            max = get_max_num(&zeros, &nums, digit_num + 1, false);
        }
    }

    // println!("max: {}, min: {}", max, min);
    let mut candidate = Vec::new();
    if max != -1 {
        candidate.push(get_clicked_button_num(&target, &max));
    }
    if min != -1 {
        candidate.push(get_clicked_button_num(&target, &min));
    }
    candidate.push((target - 100).abs());
    let result = candidate.iter().min().unwrap();
    return *result;
}
fn main() {
    let target = read_line_as_numbers()[0];
    let n = read_line_as_numbers()[0];
    let removing = read_line_as_numbers();
    let result = solve(target, n, removing);
    println!("{}", result);
}
