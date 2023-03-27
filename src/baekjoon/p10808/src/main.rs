fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let target = read_line_as_string();
    let mut map = std::collections::HashMap::new();

    target
        .chars()
        .for_each(|c| *map.entry(c).or_insert(0) += 1);

    let result = ('a'..='z')
        .map(|c| map.get(&c).unwrap_or(&0).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    
    println!("{}", result);
}
