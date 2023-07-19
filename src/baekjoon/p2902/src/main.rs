fn read_line_as_strings_split_by_hypen() -> Vec<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split("-").map(|x| x.to_string()).collect()
}

fn main() {
    let v = read_line_as_strings_split_by_hypen();
    let mut ans = String::new();
    for s in v {
        ans.push_str(&s[0..1]);
    }
    println!("{}", ans);
}
