use std::io;

fn get_dial_num_from_char(c: char) -> i32 {
    match c {
        'A'..='C' => 3,
        'D'..='F' => 4,
        'G'..='I' => 5,
        'J'..='L' => 6,
        'M'..='O' => 7,
        'P'..='S' => 8,
        'T'..='V' => 9,
        'W'..='Z' => 10,
        _ => 11
    }
}

fn read_line_as_swap_and_print_numbers() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let result:i32 = s
        .trim()
        .chars()
        .fold(0, |accum, x| {
            accum + get_dial_num_from_char(x)
        });

    println!("{}", result);
}

fn main() {
    read_line_as_swap_and_print_numbers();
}
