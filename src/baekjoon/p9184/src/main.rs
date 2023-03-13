use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn w(a: i32, b: i32, c: i32, memo: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    let result = 
        if a <= 0 || b <= 0 || c <= 0 {
            if memo.contains_key(&(0, 0, 0)) {
                return *memo.get(&(0, 0, 0)).unwrap();
            }
            memo.insert((0, 0, 0), 1);
            1
        } else if a > 20 || b > 20 || c > 20 {
            if memo.contains_key(&(20, 20, 20)) {
                return *memo.get(&(20, 20, 20)).unwrap();
            }
            w(20, 20, 20, memo)
        } else if a < b && b < c {
            let result1 = {
                if memo.contains_key(&(a, b, c - 1)) {
                    *memo.get(&(a, b, c - 1)).unwrap()
                } else {
                    w(a, b, c - 1, memo)
                }
            };
            let result2 = {
                if memo.contains_key(&(a, b - 1, c - 1)) {
                    *memo.get(&(a, b - 1, c - 1)).unwrap()
                } else {
                    w(a, b - 1, c - 1, memo)
                }
            };
            let result3 = {
                if memo.contains_key(&(a, b - 1, c)) {
                    *memo.get(&(a, b - 1, c)).unwrap()
                } else {
                    w(a, b - 1, c, memo)
                }
            };
            result1 + result2 - result3
        } else {
            let result1 = {
                if memo.contains_key(&(a - 1, b, c)) {
                    *memo.get(&(a - 1, b, c)).unwrap()
                } else {
                    w(a - 1, b, c, memo)
                }
            };
            let result2 = {
                if memo.contains_key(&(a - 1, b - 1, c)) {
                    *memo.get(&(a - 1, b - 1, c)).unwrap()
                } else {
                    w(a - 1, b - 1, c, memo)
                }
            };
            let result3 = {
                if memo.contains_key(&(a - 1, b, c - 1)) {
                    *memo.get(&(a - 1, b, c - 1)).unwrap()
                } else {
                    w(a - 1, b, c - 1, memo)
                }
            };
            let result4 = {
                if memo.contains_key(&(a - 1, b - 1, c - 1)) {
                    *memo.get(&(a - 1, b - 1, c - 1)).unwrap()
                } else {
                    w(a - 1, b - 1, c - 1, memo)
                }
            };
            result1 + result2 + result3 - result4
        };
    
    memo.insert((a, b, c), result);
    result
}

fn main() {
    let mut map: HashMap<(i32, i32, i32), i32> = HashMap::new();
    loop {
        let numbers = read_line_as_numbers();
        let (n1, n2, n3) = (numbers[0], numbers[1], numbers[2]);
        if n1 == -1 && n2 == -1 && n3 == -1 {
            break;
        }

        let result = w(n1, n2, n3, &mut map);
        println!("w({}, {}, {}) = {}", n1, n2, n3, result);
    }
}
