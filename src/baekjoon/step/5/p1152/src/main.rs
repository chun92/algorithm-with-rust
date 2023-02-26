use std::io;

fn read_line_and_cound_words() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().split_whitespace().collect::<Vec<&str>>().len();
    value.try_into().unwrap()
}


fn main() {
    let result = read_line_and_cound_words();
    println!("{}", result);
}
