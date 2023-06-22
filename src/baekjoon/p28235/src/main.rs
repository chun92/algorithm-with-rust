fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let result = match read_line_as_string().as_str() {
        "SONGDO" => "HIGHSCHOOL",
        "CODE" => "MASTER",
        "2023" => "0611",
        "ALGORITHM" => "CONTEST",
        _ => ""
    };

    println!("{}", result);
}
