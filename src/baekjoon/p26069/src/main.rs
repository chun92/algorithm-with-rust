use std::collections::HashSet;
fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let n = read_line_as_strings()[0].parse::<i32>().unwrap();
    let mut set = HashSet::new();
    let init = String::from("ChongChong");
    set.insert(init);

    for _ in 0..n {
        let line = read_line_as_strings();
        let (a, b) = (line[0].clone(), line[1].clone());
        
        if set.contains(&a) {
            set.insert(b);
        } else if set.contains(&b) {
            set.insert(a);
        }
    }

    println!("{}", set.len());
}
