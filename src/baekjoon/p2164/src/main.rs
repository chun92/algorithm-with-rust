fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();

    let mut vec = (1..=n).collect::<Vec<i32>>();
    let mut prev_odd = false;

    while vec.len() > 1 {
        let mut new_vec = Vec::new();
        let mode = if prev_odd { 0 } else { 1 };
        for (i, v) in vec.iter().enumerate() {
            if i % 2 == mode {
                new_vec.push(*v);
            }
        }

        if vec.len() % 2 == 1 {
            prev_odd = !prev_odd;
        }
        
        vec = new_vec;
    }

    println!("{}", vec[0]);
}
