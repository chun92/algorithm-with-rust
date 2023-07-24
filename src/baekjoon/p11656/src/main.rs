fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();
    let mut vec = Vec::new();

    for i in 0..str.len() {
        vec.push(String::from(&str[i..str.len()]));
    }

    vec.sort();

    for i in 0..vec.len() {
        println!("{}", vec[i]);
    }
}
