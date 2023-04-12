use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(n: usize, m: usize, cur: Vec<usize>, nums: &Vec<usize>, output: &mut BufWriter<std::io::Stdout>) {
    if cur.len() == m {
        writeln!(output, "{}", cur.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
        return;
    }

    for i in 0..n {
        let target = nums[i];
        if cur.contains(&target) {
            continue;
        }

        let mut new_cur = cur.clone();
        new_cur.push(target);
        solve(n, m, new_cur, nums, output);
    }
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut nums = read_line_as_numbers();
    nums.sort();

    let mut output = BufWriter::new(stdout());
    solve(n, m, vec![], &nums, &mut output);
}
