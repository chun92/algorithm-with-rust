use std::collections::BTreeMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let t = read_line_as_numbers()[0];
    let n = read_line_as_numbers()[0] as usize;
    let mut vec = Vec::new();
    let mut sum = 0;
    let line = read_line_as_numbers();
    for i in 0..n {
        sum += line[i];
        vec.push(sum);
    }

    let mut a_map: BTreeMap<i32, i32> = BTreeMap::new();
    for i in 0..n {
        for j in i..n {
            let sum = vec[j] - if i == 0 { 0 } else { vec[i - 1] };
            *a_map.entry(sum).or_insert(0) += 1;
        }
    }
    
    let n = read_line_as_numbers()[0] as usize;
    let mut vec = Vec::new();
    let mut sum = 0;
    let line = read_line_as_numbers();
    for i in 0..n {
        sum += line[i];
        vec.push(sum);
    }

    let mut b_map = BTreeMap::new();
    for i in 0..n {
        for j in i..n {
            let sum = vec[j] - if i == 0 { 0 } else { vec[i - 1] };
            *b_map.entry(sum).or_insert(0) += 1;
        }
    }

    let mut sum = 0;
    for (k, v) in a_map {
        if let Some(&v2) = b_map.get(&(t - k)) {
            sum += v as u64 * v2 as u64;
        }
    }
    println!("{}", sum);
}
