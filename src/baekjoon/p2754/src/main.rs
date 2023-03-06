fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let input = read_line_as_string();
    let grade = input.trim();
    
    let result = match grade {
        "A+" => 4.3,
        "A0" => 4.0,
        "A-" => 3.7,
        "B+" => 3.3,
        "B0" => 3.0,
        "B-" => 2.7,
        "C+" => 2.3,
        "C0" => 2.0,
        "C-" => 1.7,
        "D+" => 1.3,
        "D0" => 1.0,
        "D-" => 0.7,
        "F" => 0.0,
        _ => 0.0,
    };

    println!("{:.1}", result);
}
