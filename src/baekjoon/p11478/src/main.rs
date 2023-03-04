fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_num_of_unique_sub_strings(str: &str, size: usize) -> usize {
    let mut set = std::collections::HashSet::new();
    for i in 0..str.len() - size + 1 {
        set.insert(&str[i..i + size]);
    }
    set.len()
}

fn main() {
    let str = read_line_as_string();
    let mut sum = 0;
    for i in 1..=str.len() {
        sum += get_num_of_unique_sub_strings(&str, i);
    }
    println!("{}", sum);
}
