fn read_lien_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (m, d) = {
        let v = read_lien_as_numbers();
        (v[0], v[1])
    };

    let monthes = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days = d - 1;
    for i in 0..m - 1 {
        days += monthes[i];
    }

    let ans = match days % 7 {
        0 => "MON",
        1 => "TUE",
        2 => "WED",
        3 => "THU",
        4 => "FRI",
        5 => "SAT",
        6 => "SUN",
        _ => unreachable!(),
    };

    println!("{}", ans);
}
