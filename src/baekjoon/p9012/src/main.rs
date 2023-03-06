fn read_line_as<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn check_ps(s: &String) -> bool {
    let mut p_count = 0;
    for c in s.chars() {
        match c {
            '(' => p_count += 1,
            ')' => p_count -= 1,
            _ => panic!("Invalid character"),
        }

        if p_count < 0 {
            return false;
        }
    }

    if p_count == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let n: usize = read_line_as();
    for _ in 0..n {
        let s: String = read_line_as();
        if check_ps(&s) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
