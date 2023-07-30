use core::panic;

fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let first = read_line_as_string();
    let second = read_line_as_string();
    let third = read_line_as_string();
    let first_value = match first.as_str() {
        "black" => 0,
        "brown" => 1,
        "red" => 2,
        "orange" => 3,
        "yellow" => 4,
        "green" => 5,
        "blue" => 6,
        "violet" => 7,
        "grey" => 8,
        "white" => 9,
        _ => panic!("Invalid color"),
    };
    let second_value = match second.as_str() {
        "black" => 0,
        "brown" => 1,
        "red" => 2,
        "orange" => 3,
        "yellow" => 4,
        "green" => 5,
        "blue" => 6,
        "violet" => 7,
        "grey" => 8,
        "white" => 9,
        _ => panic!("Invalid color"),
    };
    let third_value = match third.as_str() {
        "black" => 1,
        "brown" => 10,
        "red" => 100,
        "orange" => 1000,
        "yellow" => 10000,
        "green" => 100000,
        "blue" => 1000000,
        "violet" => 10000000,
        "grey" => 100000000,
        "white" => 1000000000,
        _ => panic!("Invalid color"),
    };

    let sum: u64 = (first_value * 10 + second_value) * third_value;
    println!("{}", sum);
}
