fn read_line_as_i32() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn read_line_as_i16() -> i16 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i16>().unwrap()
}

fn main() {
    let n = read_line_as_i32();
    let mut vec = Vec::new();
    for _ in 0..n {
        let num = read_line_as_i16();
        vec.push(num);
    }
    vec.sort();

    let mut mean:i64 = 0;
    let mut median:i16 = 0;
    let mut mode:Vec<i16> = Vec::new();
    let mut current_num:i16 = 0;
    let mut max_count:i64 = 0;
    let mut current_count:i64 = 0;
    let range:i16 = vec[n as usize - 1] - vec[0];
    vec
        .iter()
        .enumerate()
        .for_each(|(i, &num)| {
            mean += num as i64;
            if i == ((n - 1) as usize/ 2) {
                median = num;
            }
            if num == current_num {
                current_count += 1;
            } else {
                current_num = num;
                current_count = 1;
            }

            if current_count == max_count {
                mode.push(num);
            } else if current_count > max_count {
                mode.clear();
                mode.push(num);
                max_count = current_count;
            }
        });

    let mode_result = {
        if mode.len() > 1 {
            mode[1]
        } else {
            mode[0]
        }
    };

    println!("{}", (mean as f64 / n as f64).round() as i64);
    println!("{}", median);
    println!("{}", mode_result);
    println!("{}", range);
}
