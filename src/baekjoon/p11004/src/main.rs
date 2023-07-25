fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let (_n, k) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1] as usize)
    };
    let mut v = read_line_as_numbers();
    v.sort_unstable();
    println!("{}", v[k - 1]);
}
