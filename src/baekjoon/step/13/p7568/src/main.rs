fn read_number_as_numbers() -> Vec<u8> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_number_as_numbers()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        let nums = read_number_as_numbers();
        vec.push((nums[0], nums[1]));
    }

    let results = vec
        .iter()
        .map(|(x, y)| {
            vec
                .iter()
                .fold(1, |acc, (x2, y2)| if x < x2 && y < y2 { acc + 1 } else { acc })
        })
        .collect::<Vec<_>>();

    for (i, v) in results.iter().enumerate() {
        if i == 0 {
            print!("{}", v);
        } else {
            print!(" {}", v);
        }
    }
    println!();
}
