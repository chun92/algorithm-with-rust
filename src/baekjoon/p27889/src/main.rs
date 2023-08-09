fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let s = read_line_as_string();
    match s.as_str() {
        "NLCS" => println!("North London Collegiate School"),
        "BHA" => println!("Branksome Hall Asia"),
        "KIS" => println!("Korea International School"),
        "SJA" => println!("St. Johnsbury Academy"),
        _ => panic!("Invalid school code")
    }
}
