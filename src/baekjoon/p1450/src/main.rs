fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn all_combination(vec: &[u64], c: u64) -> Vec<u64> {
    let mut counts = Vec::new();

    for i in 0..(1 << vec.len()) {
        let sum: u64 = (0..vec.len())
            .filter(|&j| i & (1 << j) != 0)
            .map(|j| vec[j])
            .sum();
        if sum <= c {
            counts.push(sum);
        }
    }

    counts.sort();
    counts
}

fn main() {
    let (n, c) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1])
    };

    let nums = read_line_as_numbers();
    let (left, right) = nums.split_at(n / 2);
    let (left, right) = (left.to_vec(), right.to_vec());
    let vec1 = all_combination(&left, c);
    let vec2 = all_combination(&right, c);

    let mut ans = 0;
    let mut j = vec2.len();
    for &x in vec1.iter() {
        while j > 0 && x + vec2[j-1] > c {
            j -= 1;
        }
        ans += j;
    }

    println!("{}", ans);
}