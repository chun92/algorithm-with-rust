fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    loop {
        let strs = read_line_as_strings();
        let (name, age, weights) = (&strs[0], &strs[1].parse::<u32>().unwrap(), &strs[2].parse::<u32>().unwrap());

        if name == "#" && age == &0 && weights == &0 {
            break;
        }

        let status = if age > &17 || weights >= &80 {
            "Senior"
        } else {
            "Junior"
        };

        println!("{} {}", name, status);
    }
}
