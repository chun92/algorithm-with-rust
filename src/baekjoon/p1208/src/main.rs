use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn all_combination(vec: &Vec<i64>) -> HashMap<i64, i64> {
    let n = vec.len();
    let mut map = HashMap::new();
    for i in 0..n {
        let mut indexes = (0..=i).collect::<Vec<usize>>();
        loop {
            let mut sum = 0;
            for j in indexes.iter() {
                sum += vec[*j];
            }
            map.entry(sum).and_modify(|e| *e += 1).or_insert(1);
            if indexes[0] == n - i - 1 {
                break;
            }
            for j in (0..=i).rev() {
                if indexes[j] < n - 1 - (i - j) {
                    indexes[j] += 1;
                    for k in j + 1..=i {
                        indexes[k] = indexes[k - 1] + 1;
                    }
                    break;
                }
            }
        }
    }
    map
}

fn main() {
    let (n, s) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1])
    };

    let nums = read_line_as_numbers();
    let (left, right) = nums.split_at(n / 2);
    let (left, right) = (left.to_vec(), right.to_vec());
    let map1 = all_combination(&left);
    let map2 = all_combination(&right);

    let mut ans = 0;
    for (k, v) in map1.iter() {
        if let Some(v2) = map2.get(&(s - k)) {
            ans += v * v2;
        }
    }

    if let Some(v) = map1.get(&s) {
        ans += v;
    }

    if let Some(v) = map2.get(&s) {
        ans += v;
    }

    println!("{}", ans);
}
