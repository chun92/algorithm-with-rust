#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_0() {
        // 1, 2, 2, 3, 3, 4, 6, 6, 9
        assert_eq!(get_kth(3, 2), (2, 3));
    }

    #[test]
    fn test_get_1() {
        // 1, 2, 2, 3, 3, 4, 6, 6, 9
        assert_eq!(get_kth(3, 5), (6, 0));
    }

    #[test]
    fn test_get_2() {
        // 1, 2, 2, 3, 3, 4, 6, 6, 9
        assert_eq!(get_kth(3, 6), (7, 8));
    }

    #[test]
    fn test_get_3() {
        // 1, 2, 2, 3, 3, 4, 6, 6, 9
        assert_eq!(get_kth(3, 7), (8, 0));
    }

    #[test]
    fn test_get_4() {
        // 1, 2, 2, 3, 3, 4, 6, 6, 9
        assert_eq!(get_kth(3, 9), (9, 9));
    }

    #[test]
    fn test_sovle_1() {
        assert_eq!(solve(3, 7), 6);
    }
}

fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn get_kth(n: u64, num: u64) -> (u64, u64) {
    let mut count = 0;
    let mut eq_count = 0;
    for i in 1..=n {
        let k = num / i;
        if k > n {
            count += n;
        } else {
            if num % i == 0 {
                eq_count += 1;
            }
            count += k;
        }
    }
    if eq_count == 0 {
        (count, 0)
    } else {
        (count - eq_count + 1, count)
    }
}

fn solve(n: u64, k: u64) -> u64 {
    let mut min = 1;
    let mut max = std::cmp::min(n * n, 10_u64.pow(9));
    while min <= max {
        let mid = (min + max) / 2;
        let (c, e) = get_kth(n, mid);

        if e == 0 {
            if c < k {
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        } else {
            if c <= k && k <= e {
                return mid;
            } else if k < c {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }
    }
    0
}

fn main() {
    let n = read_line_as_number();
    let k = read_line_as_number();

    println!("{}", solve(n, k));
}
